"""Git three-way merge driver for flat launcher translation JSON tables."""
import json
from pathlib import Path
import sys


def table(path):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"duplicate translation key: {key}")
            result[key] = value
        return result

    result = json.loads(Path(path).read_text(encoding="utf-8"), object_pairs_hook=unique)
    if not isinstance(result, dict) or any(not isinstance(v, str) for v in result.values()):
        raise ValueError("expected a flat translation table of strings")
    return result


def merge(base, ours, theirs):
    missing = object()
    result, conflicts = {}, []
    for key in dict.fromkeys([*theirs, *ours, *base]):
        before, local, incoming = (t.get(key, missing) for t in (base, ours, theirs))
        if local == incoming or incoming == before:
            value = local
        elif local == before:
            value = incoming
        else:
            conflicts.append(key)
            continue
        if value is not missing:
            result[key] = value
    return result, conflicts


def main(args=None):
    args = sys.argv[1:] if args is None else args
    if len(args) != 3:
        print("usage: merge_lang.py BASE OURS THEIRS", file=sys.stderr)
        return 2
    try:
        base, ours, theirs = (table(path) for path in args)
        result, conflicts = merge(base, ours, theirs)
        if conflicts:
            print("Conflicting translations: " + ", ".join(conflicts), file=sys.stderr)
            return 1
        Path(args[1]).write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n",
                                 encoding="utf-8", newline="\n")
    except (OSError, ValueError) as error:
        print(f"Translation merge failed: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
