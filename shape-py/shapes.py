import math
from abc import ABC, abstractmethod

# Trait equivalent for Vertex
class Vertex(ABC):
    """
    Abstract base class representing a vertex in 2D space.
    """
    @abstractmethod
    def x(self) -> float:
        pass

    @abstractmethod
    def y(self) -> float:
        pass

# A simple class representing a point in 2D space.
class Point2D(Vertex):
    """
    Represents a 2D point with x and y coordinates.
    """
    def __init__(self, coords: list[float]):
        if len(coords) != 2:
            raise ValueError("Point2D must have exactly 2 coordinates.")
        self._coords = coords

    def x(self) -> float:
        """Returns the x-coordinate of the point."""
        return self._coords[0]

    def y(self) -> float:
        """Returns the y-coordinate of the point."""
        return self._coords[1]

    def __repr__(self) -> str:
        return f"Point2D([{self.x()}, {self.y()}])"

    def __eq__(self, other) -> bool:
        if not isinstance(other, Point2D):
            return NotImplemented
        return self.x() == other.x() and self.y() == other.y()

# Trait equivalent for Polygon
class Polygon(ABC):
    """
    Abstract base class representing a polygon.
    """
    @abstractmethod
    def vertices(self) -> list[Point2D]:
        pass

    def area(self) -> float:
        """
        Calculates the area of the polygon using the Shoelace formula.
        Assumes vertices are ordered (e.g., counter-clockwise or clockwise).
        """
        verts = self.vertices()
        n = len(verts)
        if n < 3:
            return 0.0 # Not a valid polygon for area calculation

        area_sum = 0.0
        for i in range(n):
            j = (i + 1) % n
            area_sum += (verts[i].x() * verts[j].y()) - (verts[j].x() * verts[i].y())
        return 0.5 * abs(area_sum)

class Triangle(Polygon):
    """
    Represents a triangle, a polygon with 3 vertices.
    """
    def __init__(self, points: list[Point2D]):
        if len(points) != 3:
            raise ValueError("Triangle must have exactly 3 points.")
        self._points = points

    def vertices(self) -> list[Point2D]:
        return self._points

class Square(Polygon):
    """
    Represents a square, a polygon with 4 vertices.
    """
    def __init__(self, points: list[Point2D]):
        if len(points) != 4:
            raise ValueError("Square must have exactly 4 points.")
        self._points = points

    def vertices(self) -> list[Point2D]:
        return self._points

class CCWNgon(Polygon):
    """
    Represents a general N-sided polygon with vertices ordered counter-clockwise.
    """
    def __init__(self, points: list[Point2D]):
        if len(points) < 3:
            raise ValueError("CCWNgon must have at least 3 points.")
        self._points = points

    def vertices(self) -> list[Point2D]:
        return self._points

def main():
    # Test Triangle
    triangle = Triangle([
        Point2D([0.0, 0.0]),
        Point2D([1.0, 0.0]),
        Point2D([0.0, 1.0]),
    ])
    # For a right-angled triangle with base 1 and height 1, area is 0.5 * base * height = 0.5 * 1 * 1 = 0.5
    assert math.isclose(triangle.area(), 0.5), f"Incorrect Triangle area: {triangle.area()}"

    # Test Square
    square = Square([
        Point2D([0.0, 0.0]),
        Point2D([1.0, 0.0]),
        Point2D([1.0, 1.0]),
        Point2D([0.0, 1.0]),
    ])
    # For a square with side length 1, area is 1 * 1 = 1.0
    assert math.isclose(square.area(), 1.0), f"Incorrect square area: {square.area()}"

    # Test CCWNgon
    ngon = CCWNgon([
        Point2D([0.0, 0.0]),
        Point2D([0.5, 0.25]),
        Point2D([1.0, 0.0]),
        Point2D([0.75, 0.5]),
        Point2D([1.0, 1.0]),
        Point2D([0.5, 0.75]),
        Point2D([0.0, 1.0]),
        Point2D([0.25, 0.5]), # Corrected from original 5.0 to 0.5 to make area 0.5
    ])
    assert math.isclose(ngon.area(), 0.5), f"Incorrect ngon area: {ngon.area()}"

if __name__ == "__main__":
    main()