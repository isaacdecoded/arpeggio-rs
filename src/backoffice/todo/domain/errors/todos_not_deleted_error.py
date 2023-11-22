class TodosNotDeletedError(BaseException):
    def __init__(self, msg: str):
        super().__init__(f"Unable to delete Todos due to: ${msg}")
