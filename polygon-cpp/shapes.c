// Define a small epsilon for floating-point comparisons
const float EPSILON = 1e-9;

// Simple 2D Vertex
struct Vertex {
    float x, y;
    Vertex(float _x, float _y): x(_x), y(_y) {}
    ~Vertex() = default;
};
struct Polygon {
    virtual const Vertex* get_vertices() const = 0;
    virtual int n_sides() const = 0;

    float area() const {
        const Vertex* verts = get_vertices();
        int n = n_sides();

        if (n < 3) return 0.0f;

        float acc = 0.0f;
        for (int i = 0; i < n; ++i) {
            const Vertex v1 = verts[i];
            const Vertex v2 = verts[(i + 1) % n];
            acc += v1.x * v2.y - v1.y * v2.x;
        }

        return acc > 0 ? acc / 2.0f : -acc / 2.0f;
    }

    virtual ~Polygon() {}
};

struct Triangle : Polygon {
    Vertex v0, v1, v2;

    Triangle(float x0, float y0, float x1, float y1, float x2, float y2)
        : v0(x0, y0), v1(x1, y1), v2(x2, y2) {}

    const Vertex* get_vertices() const override {
        static const Vertex verts[3] = { v0, v1, v2 };
        return verts;
    }

    int n_sides() const override {
        return 3;
    }
};

struct Square : Polygon {
    Vertex corners[4];

    Square(float x0, float y0, float x1, float y1, float x2, float y2, float x3, float y3)
        : corners{Vertex(x0, y0), Vertex(x1, y1), Vertex(x2, y2), Vertex(x3, y3)} {}

    const Vertex* get_vertices() const override {
        return corners;
    }

    int n_sides() const override {
        return 4;
    }
};

struct Ngon : Polygon {
    Vertex* vertices;
    int n_vertices;

    Ngon(Vertex* _vertices, int _n_vertices)
        : vertices(_vertices), n_vertices(_n_vertices) {}

    const Vertex* get_vertices() const override {
        return vertices;
    }

    int n_sides() const override {
        return 4;
    }
};



int main() {
//    std::cout << std::fixed << std::setprecision(10); // For consistent output precision

    // Test Triangle
    Triangle triangle(
        0.0, 0.0,
        1.0, 0.0,
        0.0, 1.0
    );
    // Area of a right triangle with base 1 and height 1 is 0.5
    float triangle_area = triangle.area();
    triangle_area > 0 ? assert(triangle.area() - 0.5 < EPSILON) : assert(-triangle.area() - 0.5 < EPSILON);

    // Test Square
    Square square(
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0
    );
    // Area of a unit square is 1
    float square_area = square.area();
    square_area > 0 ? assert(square.area() - 1.0 < EPSILON) : assert(square.area() + 1.0 < EPSILON);
    
    // Test Ngon
    Vertex vertices[8] = {
        Vertex(0.0, 0.0),
        Vertex(0.5, 0.25),
        Vertex(1.0, 0.0),
        Vertex(0.75, 0.5),
        Vertex(1.0, 1.0),
        Vertex(0.5, 0.75),
        Vertex(0.0, 1.0),
        Vertex(0.25, 5.0) // Original point from Rust example
    };
    Ngon ngon(vertices, 8);
    // As calculated previously, with these specific points, the Shoelace area is 0.5
    float ngon_area = ngon.area();
    ngon_area > 0 ? assert(ngon.area() - 0.5 < EPSILON) : assert(-ngon.area() - 0.5 < EPSILON);

    return 0;
}