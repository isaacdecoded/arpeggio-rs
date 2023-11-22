class TodoNotFoundError(BaseException):
    def __init__(self, id: str, msg: str | None = None):
        super().__init__(
            f"Todo with id <{id}> {f'not found due to: {msg}' if msg else 'not exist.'}"
        )
