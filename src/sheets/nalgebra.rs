// nalgebra Linear Algebra Cheat Sheet

// -----------------------------
// 1. BASIC VECTORS AND MATRICES
// -----------------------------

use nalgebra::*;

// Vector Creation
let vec2 = Vector2::new(1.0, 2.0);
let vec3 = Vector3::new(1.0, 2.0, 3.0);
let vec4 = Vector4::new(1.0, 2.0, 3.0, 4.0);

// Dynamic Vector
let dyn_vec = DVector::from_vec(vec![1.0, 2.0, 3.0]);

// Matrix Creation
let mat2 = Matrix2::new(
    1.0, 2.0,
    3.0, 4.0
);

let mat3 = Matrix3::new(
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
    7.0, 8.0, 9.0
);

// Identity Matrices
let id2 = Matrix2::identity();
let id3 = Matrix3::identity();

// Zero and Ones
let zeros = Matrix3::zeros();
let ones = Matrix3::from_element(1.0);

// -----------------------------
// 2. VECTOR OPERATIONS
// -----------------------------

// Basic Operations
let v1 = Vector2::new(1.0, 2.0);
let v2 = Vector2::new(3.0, 4.0);

let sum = v1 + v2;
let diff = v1 - v2;
let scaled = v1 * 2.0;

// Dot Product
let dot_product = v1.dot(&v2);

// Cross Product (3D only)
let a = Vector3::new(1.0, 0.0, 0.0);
let b = Vector3::new(0.0, 1.0, 0.0);
let cross_product = a.cross(&b);

// Normalization
let normalized = v1.normalize();
let norm = v1.norm();  // Length/magnitude
let norm_squared = v1.norm_squared();

// Component Access
let x = v1[0];  // First component
let y = v1[1];  // Second component

// -----------------------------
// 3. MATRIX OPERATIONS
// -----------------------------

// Basic Operations
let m1 = Matrix2::new(1.0, 2.0, 3.0, 4.0);
let m2 = Matrix2::new(5.0, 6.0, 7.0, 8.0);

let sum = m1 + m2;
let diff = m1 - m2;
let scaled = m1 * 2.0;

// Matrix Multiplication
let product = m1 * m2;
let vec_transform = m1 * v1;  // Matrix-vector multiplication

// Transpose
let transposed = m1.transpose();

// Inverse
let inverse = m1.try_inverse().unwrap();

// Determinant
let det = m1.determinant();

// Trace
let trace = m1.trace();

// Element Access
let element = m1[(0, 1)];  // Row 0, Column 1

// -----------------------------
// 4. TRANSFORMATIONS
// -----------------------------

// 2D Transformations
let rotation = Rotation2::new(std::f64::consts::PI / 4.0);  // 45 degrees
let translation = Translation2::new(1.0, 2.0);
let scale = Scale2::new(2.0, 3.0);

// 3D Transformations
let rotation3d = Rotation3::from_euler_angles(
    0.0,  // roll
    std::f64::consts::PI / 2.0,  // pitch
    0.0   // yaw
);

let translation3d = Translation3::new(1.0, 2.0, 3.0);
let scale3d = Scale3::new(2.0, 2.0, 2.0);

// Isometry (Translation + Rotation)
let iso2 = Isometry2::new(Vector2::new(1.0, 2.0), 0.0);
let iso3 = Isometry3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 0.0));

// Similarity (Translation + Rotation + Uniform Scale)
let sim2 = Similarity2::new(Vector2::new(1.0, 2.0), 0.0, 2.0);

// -----------------------------
// 5. ADVANCED OPERATIONS
// -----------------------------

// QR Decomposition
let qr = m1.qr();
let q = qr.q();
let r = qr.r();

// SVD Decomposition
let svd = m1.svd(true, true);
let u = svd.u;  // Left singular vectors
let singular_values = svd.singular_values;
let v = svd.v_t;  // Transposed right singular vectors

// Eigendecomposition
let eigen = m1.symmetric_eigen();
let eigenvalues = eigen.eigenvalues;
let eigenvectors = eigen.eigenvectors;

// LU Decomposition
let lu = m1.lu();
let l = lu.l();
let u = lu.u();
let p = lu.p();

// -----------------------------
// 6. GEOMETRIC PRIMITIVES
// -----------------------------

// Points
let point2 = Point2::new(1.0, 2.0);
let point3 = Point3::new(1.0, 2.0, 3.0);

// Converting between Point and Vector
let as_vec = point2.coords;
let back_to_point = Point2::from(as_vec);

// Unit Vectors
let unit_x = Vector3::x_axis();
let unit_y = Vector3::y_axis();
let unit_z = Vector3::z_axis();

// -----------------------------
// 7. QUATERNIONS
// -----------------------------

// Creating Quaternions
let quat = UnitQuaternion::from_euler_angles(0.0, 0.0, std::f64::consts::PI);
let quat_from_axis_angle = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 1.0);

// Quaternion Operations
let q1 = UnitQuaternion::from_euler_angles(0.1, 0.2, 0.3);
let q2 = UnitQuaternion::from_euler_angles(0.4, 0.5, 0.6);

let product = q1 * q2;  // Composition
let inverse = q1.inverse();  // Inverse rotation

// Converting to Matrix
let rotation_matrix = q1.to_rotation_matrix();

// -----------------------------
// 8. PERFORMANCE OPTIMIZATION
// -----------------------------

// Stack Allocation
let mat: Matrix<f64, U3, U3, ArrayStorage<f64, 9>> = Matrix3::identity();

// Dynamic Allocation
let dyn_mat = DMatrix::identity(100, 100);

// Matrix Slice
let slice = mat.slice((0, 0), (2, 2));

// Avoiding Temporary Allocations
let mut result = Vector3::zeros();
result.axpy(2.0, &vec3, &Vector3::zeros());  // result = 2.0 * vec3 + 0

// -----------------------------
// 9. TYPE CONVERSIONS
// -----------------------------

// Converting between Types
let array: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
let vector = Vector4::from(array);
let back_to_array = vector.as_slice();

// Converting Dimensions
let vec2d = Vector2::new(1.0, 2.0);
let vec3d = vec2d.push(3.0);  // Add component
let back_to_2d = vec3d.fixed_rows::<2>(0);  // Take first 2 components

// -----------------------------
// 10. ERROR HANDLING
// -----------------------------

// Safe Matrix Operations
let maybe_inverse = mat3.try_inverse();
match maybe_inverse {
    Some(inv) => println!("Inverse found"),
    None => println!("Matrix is not invertible"),
}

// Decomposition Checks
let svd = mat3.svd(true, true);
if svd.singular_values[0] < 1e-10 {
    println!("Matrix is numerically singular");
}

// Safe Vector Normalization
let normalized = vec3.try_normalize(1e-10)
    .unwrap_or_else(|| Vector3::zeros());