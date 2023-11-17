def bindings(cd: str, args: list[str]) -> str | int | None:
    from libs.llm.bindings.bindings import build_llm_bindings

    return build_llm_bindings(cd, args)
