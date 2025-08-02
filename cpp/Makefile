# Define the C++ compiler to use
CXX = g++

# Define compiler flags
# -std=c++17: Use the C++17 standard
# -Wall: Enable all standard warnings
# -Wextra: Enable extra warnings
# -O2: Enable a moderate level of optimization
CXXFLAGS = -std=c++17 -Wall -Wextra -O2

# The name of the executable file to be created
TARGET = shapes_app

# The source file(s) for the project
SRCS = shapes.cpp

# The default target. 'make' will build this.
# It depends on the executable being present.
all: $(TARGET)

# Rule to build the executable from the source file(s)
$(TARGET): $(SRCS)
	$(CXX) $(CXXFLAGS) $(SRCS) -o $(TARGET)

# A phony target is a target that doesn't correspond to a real file.
.PHONY: all clean

# A target to clean up the build artifacts
clean:
	rm -f $(TARGET)