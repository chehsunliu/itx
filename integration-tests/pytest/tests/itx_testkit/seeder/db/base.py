from abc import ABC, abstractmethod
from pathlib import Path


class DbReader(ABC):
    pass


class DbSeeder(ABC):
    @abstractmethod
    async def reset_tables(self): ...

    @abstractmethod
    async def write_data(self, folder_path: Path): ...

    @abstractmethod
    def reader(self) -> DbReader: ...
