#!/usr/bin/env python3
import argparse
import re
import subprocess
import tempfile
from pathlib import Path


FENCED_BLOCK_RE = re.compile(r"^```rust\b.*?\n(.*?)^```$", re.MULTILINE | re.DOTALL)


def process(input_root: Path, output_root: Path) -> None:
    num_files = 0
    num_blocks = 0

    for md_path in sorted(input_root.rglob("*.md")):
        rel_path = md_path.relative_to(input_root)
        content = md_path.read_text(encoding="utf-8")

        blocks = re.findall(FENCED_BLOCK_RE, content)
        if not blocks:
            continue

        num_files += 1
        num_blocks += len(blocks)

        out_dir = output_root / rel_path.parent
        out_dir.mkdir(parents=True, exist_ok=True)
        for num, body in enumerate(blocks, 1):
            text = (
                f"// source: patina-by-example/{rel_path.as_posix()}\n"
                f"// block: {num}/{len(blocks)}\n"
                f"\n{body}"
            )

            out_path = out_dir / f"{rel_path.stem}__{num:02d}.pt"
            out_path.write_text(text, encoding="utf-8")

    return num_files, num_blocks


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Extract code blocks from markdown into .pt test files."
    )
    parser.add_argument("input", help="input directory path or git repository URL")
    parser.add_argument("output", help="output directory")
    args = parser.parse_args()

    output_root = Path(args.output)
    source = Path(args.input)

    if source.exists():
        num_files, num_blocks = process(source, output_root)
    else:
        with tempfile.TemporaryDirectory() as tmpdir:
            subprocess.run(
                ["git", "clone", "--depth=1", args.input, tmpdir], check=True
            )
            num_files, num_blocks = process(Path(tmpdir), output_root)

    print(f"extracted {num_blocks} blocks from {num_files} files into {output_root}")


if __name__ == "__main__":
    main()
