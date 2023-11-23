def bindings(cd: str, args: list[str]) -> str | int | None:
    from libs.l2math.bindings.bindings import build_l2math_bindings

    return build_l2math_bindings(cd, args)
