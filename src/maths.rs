
const PI: f64 = 3.14159;

// #define ToRadian(x) ((x) * 3.14159f /180.0f)
pub fn to_radian(x: f64) -> f64 {
    x * PI / 180.0
}

// #define ToDegree(x) ((x) * 180.0f / 3.14159f)
pub fn to_degree(x: f64) -> f64 {
    x * 180.0 / PI
}

// inline float LinearMap(float x, float fromMin, float fromMax, float toMin, float toMax)
// {
//    return toMin + ((x - fromMin) / (fromMax - fromMin)) * (toMax - toMin);
//}
pub fn linear_map(x: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    to_min + ((x - from_min) / (from_max - from_min)) * (to_max - to_min)
}

// inline int mod_floor(int a, int n) {
//     return ((a % n) + n) % n;
// }
// inline int mod_floor(int a, size_t n) {
//     return mod_floor(a, (int)n);
// }
pub fn mod_floor(a: i32, n: i32) {
    ((a % n) + n) % n
}

/* template<typename T>
inline T clamp(T val, T min, T max)
{
    if (val < min)
        return min;
    if (val > max)
        return max;
    return val;
}

double generateGaussian(double mean, double stdDev)
{
    // Implementation of Marsaglia polar method. Generates two normally distributed random variables in the range of 0-1
    // Thread local storage is used to return the alternate of the two random variables every other call
    
    thread_local double spare;
    thread_local bool hasSpare = false;

    if (hasSpare) 
    {
        hasSpare = false;
        return spare * stdDev + mean;
    }
    else {
        double u, v, s;
        do 
        {
            u = (rand() / ((double)RAND_MAX)) * 2.0 - 1.0;
            v = (rand() / ((double)RAND_MAX)) * 2.0 - 1.0;
            s = u * u + v * v;
        } 
        while (s >= 1.0 || s == 0.0);

        s = sqrt(-2.0 * log(s) / s);
        spare = v * s;
        hasSpare = true;

        return mean + stdDev * u * s;
    }
} */