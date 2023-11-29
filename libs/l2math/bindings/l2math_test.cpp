#include "l2math.h"
#include "EZtest"

#include <iostream>

using namespace l2math;

constexpr const Float64 THIRTY_DEGREES = 0.5235987756;
constexpr const Float64 SIXTY_DEGREES =  1.0471975512;

TEST(main_test) {
    Float64 thirty = DEG2RAD(30.0);
    Float64 sixty = DEG2RAD(60.0);

    ASSERT_EQ(
        round(thirty, 10),
        round(THIRTY_DEGREES, 10)
    );
    ASSERT_EQ(
        round(sixty, 10),
        round(SIXTY_DEGREES, 10)
    );

    Float64 one_half_1 = l2math::sin(thirty);
    Float64 one_half_2 = l2math::cos(sixty);
    one_half_1 = round(one_half_1, 10);
    one_half_2 = round(one_half_2, 10);

    ASSERT_EQ(one_half_1, one_half_2);
    OK
}

int main(int argc, char** argv) {
    return RUN_TESTS;
}
