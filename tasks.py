# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "typer==0.20.0",
#     "beautifulsoup4==4.12.2",
#     "browser_cookie3==0.16.2",
#     "html2text==2020.1.16",
#     "python-dotenv==1.0.0",
#     "requests==2.27.1",
#     "tabulate==0.9.0",
#     "termcolor==1.1.0",
#     "tomlkit==0.12.3",
# ]
# ///
import re
import shlex
import subprocess
import sys
import time
import typing as t
import webbrowser
from contextlib import chdir
from datetime import datetime, timedelta
from functools import partial, wraps
from os import environ, startfile
from pathlib import Path
from uuid import uuid4

import browser_cookie3
import html2text
import requests
import tomlkit as toml
import typer
from bs4 import BeautifulSoup
from bs4.element import Tag
from dotenv import load_dotenv
from termcolor import colored as c


class AliasGroup(typer.core.TyperGroup):
    _CMD_SPLIT_P = re.compile(r" ?[,|] ?")

    def get_command(self, ctx, cmd_name):
        cmd_name = self._group_cmd_name(cmd_name)
        return super().get_command(ctx, cmd_name)

    def _group_cmd_name(self, default_name):
        for cmd in self.commands.values():
            name = cmd.name
            if name and default_name in self._CMD_SPLIT_P.split(name):
                return name
        return default_name


app = typer.Typer(cls=AliasGroup, context_settings={"help_option_names": ["-h", "--help"]})
cb = partial(c, attrs=["bold"])


def handle_errors(errors: t.Type[BaseException] | tuple[t.Type[BaseException], ...]):
    def decorator(f):
        @wraps(f)
        def inner(*args, **kwargs):
            try:
                return f(*args, **kwargs)
            except errors as e:
                print(cb(f"Error: {e}", "red"))
                sys.exit(1)

        return inner

    return decorator


MAIN = """\
fn main() {{
    let (part1, part2) = {crate}::solve();
    println!("{{part1}}");
    println!("{{part2}}");
}}\
"""

LIB = """\
use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    ("TODO", "TODO")
}\
"""

WORKSPACE_MANIFEST_PATH = Path(__file__).parent / "Cargo.toml"

NOW = datetime.now()
YEAR = toml.parse(WORKSPACE_MANIFEST_PATH.read_text()).get("metadata", {}).get("year", NOW.year)

load_dotenv()


def _build_session() -> requests.Session:
    session = requests.Session()
    if "SESSION_COOKIE" in environ:
        session.cookies.update({"session": environ["SESSION_COOKIE"]})
    else:
        session.cookies.update(browser_cookie3.firefox(domain_name="adventofcode.com"))
    # TODO: more identifying user-agent
    session.headers.update({"User-Agent": "PurpleMyst/aoc-template with much love! <3"})
    return session


session = _build_session()


def run(cmd: t.Sequence[str | Path], /, **kwargs) -> subprocess.CompletedProcess:
    check = kwargs.pop("check", True)
    print(
        cb("$", "green"),
        shlex.join(map(str, cmd)),
        c(f"(w/ options {kwargs})", "green") if kwargs else "",
    )
    proc = subprocess.run(cmd, **kwargs)
    if check and proc.returncode != 0:
        print(cb("Failed.", "red"))
        sys.exit(proc.returncode)
    return proc


def add_line(p: Path, l: str) -> None:
    ls = p.read_text().splitlines()
    ls.insert(-1, l)
    if ls[-1] != "":
        # Add or keep trailing newline
        ls.append("")
    p.write_text("\n".join(ls), newline="\n")


def in_root_dir(f):
    @wraps(f)
    def inner(*args, **kwargs):
        with chdir(Path(__file__).parent):
            return f(*args, **kwargs)

    return inner


def find_next_day() -> int | None:
    existing_days = []
    for path in Path().glob("day*"):
        if path.name.startswith("day") and path.name[3:].isdigit():
            existing_days.append(int(path.name[3:]))
    next_day = 1
    while next_day in existing_days:
        next_day += 1
    if next_day > 25:
        return None
    return next_day


@app.command("refetch-inputs")
@handle_errors((requests.HTTPError,))
def refetch_inputs() -> None:
    "Fetch the inputs that aren't present locally."
    for day in Path(__file__).parent.glob("day*"):
        input_path = day / "src" / "input.txt"
        if input_path.exists():
            continue
        day_num = int(day.name.removeprefix("day"))
        print(f"Fetching input for day {day_num}...")
        resp = session.get(f"https://adventofcode.com/{YEAR}/day/{day_num}/input")
        resp.raise_for_status()
        input_path.write_text(resp.text, newline="\n")


@app.command("start-solve | ss")
@handle_errors((requests.HTTPError,))
@in_root_dir
def start_solve(day: int | None = None) -> None:
    "Start solving a day. Defaults to the next available day."

    if day is None:
        day = find_next_day()
        if day is None:
            print(cb("All 25 days already exist!", "red"))
            return

    crate = f"day{day:02}"
    crate_path = Path(crate)

    if crate_path.exists():
        print(f"{crate} already exists.")
        return

    resp = session.get(f"https://adventofcode.com/{YEAR}/day/{day}/input")
    resp.raise_for_status()
    puzzle_input = resp.text

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    if crate not in manifest["workspace"]["members"]:  # type: ignore
        manifest["workspace"]["members"].append(crate)  # type: ignore

    metadata = manifest["workspace"].setdefault("metadata", {})  # type: ignore
    metadata[crate] = {"start_time": datetime.now()}

    with WORKSPACE_MANIFEST_PATH.open("w") as manifest_f:
        toml.dump(manifest, manifest_f)

    run(("cargo", "new", "--bin", crate))
    run(
        (
            "cargo",
            "add",
            "--manifest-path",
            "benchmark/Cargo.toml",
            "--path",
            crate,
            crate,
        )
    )

    src = crate_path / "src"
    (src / "main.rs").write_text(MAIN.format(crate=crate), newline="\n")
    (src / "lib.rs").write_text(LIB, newline="\n")
    (src / "input.txt").write_text(puzzle_input, newline="\n")

    benches = Path("benchmark", "benches")
    add_line(benches / "criterion.rs", f"    {crate},")
    add_line(benches / "iai.rs", f"    {crate}: {crate}_solve,")

    fetch_problem(YEAR, day)

    run(("git", "add", crate))
    webbrowser.open_new(f"https://adventofcode.com/{YEAR}/day/{day}")


@app.command("wait-start | ws")
def wait_start(day: int | None = None, hour: int = 6) -> None:
    "Waits until the next 6AM (or specified hour) then calls start-solve."

    now = datetime.now()
    # Set target to today at the specified hour (default 6 AM)
    target = now.replace(hour=hour, minute=0, second=2, microsecond=0)

    # If 6 AM has already passed today, target 6 AM tomorrow
    if target <= now:
        target += timedelta(days=1)

    delta = target - now
    seconds_to_wait = delta.total_seconds()

    print(cb(f"Current time: {now.strftime('%Y-%m-%d %H:%M:%S')}", "cyan"))
    print(cb(f"Target time:  {target.strftime('%Y-%m-%d %H:%M:%S')}", "green"))
    print(cb(f"Sleeping for: {delta}...", "yellow"))

    try:
        time.sleep(seconds_to_wait)
        print(cb("\nWake up! Starting solve task...", "green"))
        # Call the existing function directly
        start_solve(day)
    except KeyboardInterrupt:
        print(cb("\nTimer cancelled.", "red"))
        sys.exit(0)


def _is_dirty(*, ignore_untracked_files: bool = True) -> bool:
    return (
        run(
            ("git", "status", "--porcelain", "--untracked-files=no")
            if ignore_untracked_files
            else ("git", "status", "--porcelain"),
            capture_output=True,
            text=True,
        ).stdout.strip()
        != ""
    )


@app.command("set-baseline | sb")
@in_root_dir
def set_baseline(day: t.Annotated[str, typer.Argument()] = ".") -> None:
    "Run a criterion benchmark, setting its results as the new baseline (using Git hash)."

    if _is_dirty():
        print(
            cb(
                "You have uncommitted changes. Please commit or stash before setting a baseline.",
                "red",
            )
        )
        sys.exit(1)

    # Use Git hash as baseline name
    name = run(
        ("git", "rev-parse", "--short", "HEAD"), capture_output=True, text=True
    ).stdout.strip()

    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            day,
            "--save-baseline",
            name,
            "--verbose",
        )
    )


@app.command("compare | cmp")
@in_root_dir
def compare(day: t.Annotated[str, typer.Argument()]) -> None:
    "Compare benchmark results. (Dirty -> compare vs HEAD; Clean -> compare HEAD vs HEAD~1)."

    if _is_dirty(ignore_untracked_files=False):
        print(cb("Dirty state detected: Comparing current changes against HEAD.", "yellow"))
        name = run(
            ("git", "rev-parse", "--short", "HEAD"), capture_output=True, text=True
        ).stdout.strip()
    else:
        print(cb("Clean state detected: Comparing HEAD against HEAD~1.", "green"))
        name = run(
            ("git", "rev-parse", "--short", "HEAD~1"), capture_output=True, text=True
        ).stdout.strip()

    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            day,
            "--baseline",
            name,
            "--verbose",
        )
    )


@app.command("compare-by-stashing | cmp-stash")
@in_root_dir
def compare_by_stashing(day: t.Annotated[str, typer.Argument()]) -> None:
    "Stash current changes, set baseline, pop stash, then compare."
    if not _is_dirty(ignore_untracked_files=False):
        print(cb("Repository is clean; no need to stash.", "green"))
        compare(day)
        return

    if not _is_dirty():
        print(cb("No changes to tracked files; stashing would be a no-op.", "yellow"))
        return

    run(("git", "stash", "push", "-m", "Stashing for benchmarking"))
    # We recursively call set_baseline here; careful as it checks for dirty state,
    # but we just stashed, so it should be clean.
    set_baseline(day)
    run(("git", "stash", "pop"))
    compare(day)


@app.command()
@in_root_dir
def criterion(day: str) -> None:
    "Run a criterion benchmark without baselines."
    run(("cargo", "bench", "--bench", "criterion", "--", day, "--verbose"))


@app.command()
@in_root_dir
def iai() -> None:
    "Run the iai benchmark."
    run(("cargo", "bench", "--bench", "iai"))


@app.command("answer | a")
@handle_errors((requests.HTTPError, AssertionError))
def answer(answer: str, level: int) -> None:
    "Submit your answer!"

    day = Path.cwd().resolve().name
    if not day.startswith("day"):
        print(cb("Not in a day directory.", "red"))
        return

    resp = session.post(
        f"https://adventofcode.com/{YEAR}/day/{day}/answer",
        data={"answer": answer, "level": str(level)},
    )
    resp.raise_for_status()

    # Get the main text, removing the "return to" link, and show it in markdown form.
    soup = BeautifulSoup(resp.text, features="html.parser").main
    assert soup is not None, "no main element?"
    return_link = soup.find(href=f"/{YEAR}/day/{day}")
    if isinstance(return_link, Tag):
        return_link.decompose()
    h = html2text.HTML2Text()
    h.ignore_links = True
    print(h.handle(str(soup)).strip())


@in_root_dir
def fetch_problem(year, day) -> None:
    "Fetch the problem statement."
    resp = session.get(f"https://adventofcode.com/{year}/day/{day}")
    resp.raise_for_status()
    soup = BeautifulSoup(resp.text, features="html.parser").main
    h = html2text.HTML2Text()
    t = h.handle(str(soup)).strip()
    Path(f"day{day:02}", "problem.md").write_text(t, newline="\n")


@app.command()
def show_session_cookie() -> None:
    "Conquer outer space."
    print(c("Your session cookie:", "yellow"), session.cookies["session"])


@app.command("measure-completion-time | mct")
@in_root_dir
def measure_completion_time() -> None:
    "Measure completion time for all days."
    from tabulate import tabulate

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())

    table = []
    for day in Path().glob("day*"):
        day_metadata = manifest["workspace"].get("metadata", {}).get(day.name, {})  # type: ignore
        start_time = day_metadata.get("start_time")
        end_time = day_metadata.get("completion_time")
        if start_time is None or end_time is None:
            print(cb(f"Day {day.name} is missing start or end time.", "yellow"))
            completion_time = "N/A"
        else:
            completion_time = end_time - start_time
        table.append((day.name, str(completion_time)))
    print(tabulate(table, headers=["Day", "Completion Time"], tablefmt="fancy_grid"))


@app.command("set-completion-time | sct")
def set_completion_time() -> None:
    "Set the completion time for the day you're currently in."

    day = Path.cwd().resolve().name
    if not day.startswith("day"):
        print(cb("Not in a day directory.", "red"))
        return

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    metadata = manifest["workspace"].setdefault("metadata", {})  # type: ignore
    metadata.setdefault(day, {})["completion_time"] = datetime.now()

    with WORKSPACE_MANIFEST_PATH.open("w") as manifest_f:
        toml.dump(manifest, manifest_f)


@app.command()
@in_root_dir
def flamegraph(day: str, remote: str = "linode") -> None:
    "Run a flamegraph benchmark on a remote."
    import tarfile
    import tempfile

    from rich.console import Console

    console = Console()

    def filter_tar(info):
        if any(s in info.name for s in (".git", "target")):
            console.log(f"{info.name!r} [red]skipped.[/red]")
            return None
        console.log(f"{info.name!r} [green]added to tarball.[/green]")
        return info

    # Generate a zipped tarball of the current source code.
    archive_stem = str(uuid4())
    archive_name = f"{archive_stem}.tar.gz"
    with tempfile.TemporaryDirectory() as tmpdir:
        archive_path = Path(tmpdir, archive_name)
        with console.status("Compressing..."), tarfile.open(archive_path, "w:gz") as tar:
            tar.add(".", filter=filter_tar)

        # Upload it to the remote via scp and untar it.
        run(("scp", "-C", archive_path, f"{remote}:/tmp/{archive_name}"))
        run(("ssh", remote, "tar", "-xzf", f"/tmp/{archive_name}", "--one-top-level", "-C", "/tmp"))

    # Run the benchmark on the remote.
    run(
        (
            "ssh",
            remote,
            "cd",
            f"/tmp/{archive_stem}",
            "&&",
            "CARGO_PROFILE_BENCH_DEBUG=true",
            "cargo",
            "flamegraph",
            "--bench",
            "criterion",
            "--",
            "--bench",
            day,
        )
    )

    # Download the flamegraph.
    run(("scp", f"{remote}:/tmp/{archive_stem}/flamegraph.svg", "."))

    # Remove the archive from the remote.
    run(("ssh", remote, "rm", "-rf", f"/tmp/{archive_stem}", f"/tmp/{archive_name}"))

    # Open the flamegraph.
    startfile("flamegraph.svg")


def main() -> None:
    environ["RUSTFLAGS"] = "-C target-cpu=native"
    app()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Bye!")
