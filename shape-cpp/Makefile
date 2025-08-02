# Define the C++ compiler to use
CXX = g++

TARGET_PROC = x86_64-unknown-linux-gnu

# Define compiler flags
# -std=c++17: Use the C++17 standard
# -Wall: Enable all standard warnings
# -Wextra: Enable extra warnings
# -O3: Enable a aggresive optimizations
CXXFLAGS = -std=c++17 -Wall -Wextra -O3 

# The name of the executable file to be created
TARGET = shapes

# The source file(s) for the project
SRCS = shapes.cpp

# The default target. 'make' will build this.
# It depends on the executable being present.
all: $(TARGET)

# Rule to build the executable from the source file(s)
$(TARGET): $(SRCS)
	$(CXX) $(CXXFLAGS) $(SRCS) -o $(TARGET)

asm: $(SRCS)
	$(CXX) $(CXXFLAGS) -S $(SRCS) -o shape.s

# A phony target is a target that doesn't correspond to a real file.
.PHONY: all clean

# A target to clean up the build artifacts
clean:
	rm -f $(TARGET)