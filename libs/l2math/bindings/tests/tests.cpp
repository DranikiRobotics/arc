#include "../l2math.h"

int main(int argc, char** argv) {
    l2math::Float64 thirty = 30.0;
    l2math::Float64 sixty = 60.0;
    l2math::Radian64 one_half_1 = l2math::sin(thirty);
    l2math::Radian64 one_half_2 = l2math::cosf(sixty);

    if (one_half_1 != one_half_2) {
        return 1;
    }

    return 0;
}
