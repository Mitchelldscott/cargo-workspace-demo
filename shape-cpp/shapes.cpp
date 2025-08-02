#include <iostream>
#include <vector>
#include <cmath>    // For std::abs
#include <cassert>  // For assert
#include <numeric>  // For std::accumulate (though not strictly necessary here, but good for sums)
#include <iomanip>  // For std::fixed and std::setprecision

// Define a small epsilon for floating-point comparisons
const double EPSILON = 1e-9;

// Interface for Vertex (mimics Rust's Vertex trait)
class Vertex {
public:
    virtual double x() const = 0; // Pure virtual function
    virtual double y() const = 0; // Pure virtual function
    virtual ~Vertex() = default;  // Virtual destructor for proper cleanup
};

// A simple struct representing a point in 2D space.
// We make it inherit from Vertex and implement its methods.
struct Point2D : public Vertex {
    double coords[2];

    // Constructor
    Point2D(double x_val, double y_val) {
        coords[0] = x_val;
        coords[1] = y_val;
    }

    // Implementation of the Vertex methods
    double x() const override {
        return coords[0];
    }
    double y() const override {
        return coords[1];
    }
};

// Interface for Polygon (mimics Rust's Polygon trait)
class Polygon {
public:
    // Pure virtual function to get the vertices
    virtual std::vector<Point2D> vertices() const = 0;

    // Default implementation for area using the Shoelace formula
    double area() const {
        const std::vector<Point2D> verts = vertices();
        size_t n = verts.size();

        if (n < 3) {
            return 0.0; // Not a valid polygon for area calculation
        }

        double area_sum = 0.0;
        for (size_t i = 0; i < n; ++i) {
            size_t j = (i + 1) % n; // Wrap around for the last segment
            area_sum += (verts[i].x() * verts[j].y()) - (verts[j].x() * verts[i].y());
        }
        return 0.5 * std::abs(area_sum);
    }

    virtual ~Polygon() = default; // Virtual destructor
};

// Implementation of Triangle
struct Triangle : public Polygon {
    Point2D points[3];

    Triangle(Point2D p1, Point2D p2, Point2D p3) : points{p1, p2, p3} {}

    std::vector<Point2D> vertices() const override {
        return {points[0], points[1], points[2]};
    }
};

// Implementation of Square
struct Square : public Polygon {
    Point2D points[4];

    Square(Point2D p1, Point2D p2, Point2D p3, Point2D p4) : points{p1, p2, p3, p4} {}

    std::vector<Point2D> vertices() const override {
        return {points[0], points[1], points[2], points[3]};
    }
};

// Implementation of CCWNgon (Counter-Clockwise N-gon)
struct CCWNgon : public Polygon {
    std::vector<Point2D> points;

    // Constructor takes a vector of points
    CCWNgon(const std::vector<Point2D>& pts) : points(pts) {
        if (points.size() < 3)
            std::cerr << "Error: CCWNgon must have at least 3 points." << std::endl;
    }

    std::vector<Point2D> vertices() const override {
        return points;
    }
};

int main() {
    std::cout << std::fixed << std::setprecision(10); // For consistent output precision

    // Test Triangle
    Triangle triangle(
        Point2D(0.0, 0.0),
        Point2D(1.0, 0.0),
        Point2D(0.0, 1.0)
    );
    // Area of a right triangle with base 1, height 1 is 0.5
    assert(std::abs(triangle.area() - 0.5) < EPSILON);

    // Test Square
    Square square(
        Point2D(0.0, 0.0),
        Point2D(1.0, 0.0),
        Point2D(1.0, 1.0),
        Point2D(0.0, 1.0)
    );
    // Area of a square with side length 1 is 1.0
    assert(std::abs(square.area() - 1.0) < EPSILON);

    // Test CCWNgon
    CCWNgon ngon({
        Point2D(0.0, 0.0),
        Point2D(0.5, 0.25),
        Point2D(1.0, 0.0),
        Point2D(0.75, 0.5),
        Point2D(1.0, 1.0),
        Point2D(0.5, 0.75),
        Point2D(0.0, 1.0),
        Point2D(0.25, 5.0) // Original point from Rust example
    });
    // As calculated previously, with these specific points, the Shoelace area is 0.5
    assert(std::abs(ngon.area() - 0.5) < EPSILON);

    return 0;
}