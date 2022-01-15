macro_rules! impl_color_add {
    ($self_ty: ident < $phantom_ty: ident, $component_ty: ident > , [$($element: ident),+], $phantom: ident) => {
        impl<$phantom_ty, $component_ty> Add<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Add<Output=$component_ty>
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element + other.$element, )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> Add<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Add<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn add(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element + c.clone(), )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> AddAssign<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: AddAssign,
        {
            fn add_assign(&mut self, other: Self) {
                $( self.$element += other.$element; )+
            }
        }

        impl<$phantom_ty, $component_ty> AddAssign<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T:  AddAssign + Clone
        {
            fn add_assign(&mut self, c: $component_ty) {
                $( self.$element += c.clone(); )+
            }
        }
    };
    ($self_ty: ident < $component_ty: ident > , [$($element: ident),+]) => {
        impl<$component_ty> Add<Self> for $self_ty<$component_ty>
        where
            T: Add<Output=$component_ty>
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element + other.$element, )+
                }
            }
        }

        impl<$component_ty> Add<$component_ty> for $self_ty<$component_ty>
        where
            T: Add<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn add(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element + c.clone(), )+
                }
            }
        }

        impl<$component_ty> AddAssign<Self> for $self_ty<$component_ty>
        where
            T: AddAssign,
        {
            fn add_assign(&mut self, other: Self) {
                $( self.$element += other.$element; )+
            }
        }

        impl<$component_ty> AddAssign<$component_ty> for $self_ty<$component_ty>
        where
            T:  AddAssign + Clone
        {
            fn add_assign(&mut self, c: $component_ty) {
                $( self.$element += c.clone(); )+
            }
        }
    };
}

/// Implement `Sub` and `SubAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_sub {
    ($self_ty: ident < $phantom_ty: ident, $component_ty: ident > , [$($element: ident),+], $phantom: ident) => {
        impl<$phantom_ty, $component_ty> Sub<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Sub<Output=$component_ty>
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element - other.$element, )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> Sub<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Sub<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn sub(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element - c.clone(), )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> SubAssign<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: SubAssign,
        {
            fn sub_assign(&mut self, other: Self) {
                $( self.$element -= other.$element; )+
            }
        }

        impl<$phantom_ty, $component_ty> SubAssign<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T:  SubAssign + Clone
        {
            fn sub_assign(&mut self, c: $component_ty) {
                $( self.$element -= c.clone(); )+
            }
        }
    };

    ($self_ty: ident < $component_ty: ident > , [$($element: ident),+]) => {
        impl<$component_ty> Sub<Self> for $self_ty<$component_ty>
        where
            T: Sub<Output=$component_ty>
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element - other.$element, )+
                }
            }
        }

        impl<$component_ty> Sub<$component_ty> for $self_ty<$component_ty>
        where
            T: Sub<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn sub(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element - c.clone(), )+
                }
            }
        }

        impl<$component_ty> SubAssign<Self> for $self_ty<$component_ty>
        where
            T: SubAssign,
        {
            fn sub_assign(&mut self, other: Self) {
                $( self.$element -= other.$element; )+
            }
        }

        impl<$component_ty> SubAssign<$component_ty> for $self_ty<$component_ty>
        where
            T:  SubAssign + Clone
        {
            fn sub_assign(&mut self, c: $component_ty) {
                $( self.$element -= c.clone(); )+
            }
        }
    };
}

/// Implement `Mul` and `MulAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_mul {
    ($self_ty: ident < $phantom_ty: ident, $component_ty: ident > , [$($element: ident),+], $phantom: ident) => {
        impl<$phantom_ty, $component_ty> Mul<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Mul<Output=$component_ty>
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element * other.$element, )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> Mul<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Mul<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn mul(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element * c.clone(), )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> MulAssign<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: MulAssign,
        {
            fn mul_assign(&mut self, other: Self) {
                $( self.$element *= other.$element; )+
            }
        }

        impl<$phantom_ty, $component_ty> MulAssign<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T:  MulAssign + Clone
        {
            fn mul_assign(&mut self, c: $component_ty) {
                $( self.$element *= c.clone(); )+
            }
        }
    };
    ($self_ty: ident < $component_ty: ident > , [$($element: ident),+]) => {
        impl<$component_ty> Mul<Self> for $self_ty<$component_ty>
        where
            T: Mul<Output=$component_ty>
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element * other.$element, )+
                }
            }
        }

        impl<$component_ty> Mul<$component_ty> for $self_ty<$component_ty>
        where
            T: Mul<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn mul(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element * c.clone(), )+
                }
            }
        }

        impl<$component_ty> MulAssign<Self> for $self_ty<$component_ty>
        where
            T: MulAssign,
        {
            fn mul_assign(&mut self, other: Self) {
                $( self.$element *= other.$element; )+
            }
        }

        impl<$component_ty> MulAssign<$component_ty> for $self_ty<$component_ty>
        where
            T:  MulAssign + Clone
        {
            fn mul_assign(&mut self, c: $component_ty) {
                $( self.$element *= c.clone(); )+
            }
        }
    };
}

/// Implement `Div` and `DivAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_div {
    ($self_ty: ident < $phantom_ty: ident, $component_ty: ident > , [$($element: ident),+], $phantom: ident) => {
        impl<$phantom_ty, $component_ty> Div<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Div<Output=$component_ty>
        {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element / other.$element, )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> Div<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Div<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn div(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element / c.clone(), )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> DivAssign<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: DivAssign,
        {
            fn div_assign(&mut self, other: Self) {
                $( self.$element /= other.$element; )+
            }
        }

        impl<$phantom_ty, $component_ty> DivAssign<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T:  DivAssign + Clone
        {
            fn div_assign(&mut self, c: $component_ty) {
                $( self.$element /= c.clone(); )+
            }
        }
    };
    ($self_ty: ident < $component_ty: ident > , [$($element: ident),+]) => {
        impl<$component_ty> Div<Self> for $self_ty<$component_ty>
        where
            T: Div<Output=$component_ty>
        {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element / other.$element, )+
                }
            }
        }

        impl<$component_ty> Div<$component_ty> for $self_ty<$component_ty>
        where
            T: Div<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn div(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element / c.clone(), )+
                }
            }
        }

        impl<$component_ty> DivAssign<Self> for $self_ty<$component_ty>
        where
            T: DivAssign,
        {
            fn div_assign(&mut self, other: Self) {
                $( self.$element /= other.$element; )+
            }
        }

        impl<$component_ty> DivAssign<$component_ty> for $self_ty<$component_ty>
        where
            T:  DivAssign + Clone
        {
            fn div_assign(&mut self, c: $component_ty) {
                $( self.$element /= c.clone(); )+
            }
        }
    };
}
