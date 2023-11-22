class TodosNotFoundError(BaseException):
    def __init__(self, msg: str):
        super().__init__(f"Unable to fetch Todos due to: ${msg}")
