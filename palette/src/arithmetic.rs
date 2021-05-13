//! Macros to implement arithmetic traits on Color spaces.

/// Implement `Add` and `AddAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
#[macro_export]
macro_rules! impl_color_add {
    ($self_ty: ident , [$($element: ident),+], $phantom: ident) => {
	impl<Wp, T> Add<$self_ty<Wp, T>> for $self_ty<Wp, T>
	where
	    T: FloatComponent,
	    Wp: WhitePoint,
	{
	    type Output = $self_ty<Wp, T>;

	    fn add(self, other: $self_ty<Wp, T>) -> Self::Output {
		$self_ty {
		    $( $element: self.$element + other.$element ),+,
		    $phantom: PhantomData,
		}
	    }
	}
	impl<Wp, T> Add<T> for $self_ty<Wp, T>
	where
	    T: FloatComponent,
	    Wp: WhitePoint,
	{
	    type Output = $self_ty<Wp, T>;

	    fn add(self, c: T) -> Self::Output {
		$self_ty {
		    $( $element: self.$element + c ),+,
		    $phantom: PhantomData,
		}
	    }
	}

	impl<Wp, T> AddAssign<$self_ty<Wp, T>> for $self_ty<Wp, T>
	where
	    T: FloatComponent + AddAssign,
	    Wp: WhitePoint,
	{
	    fn add_assign(&mut self, other: $self_ty<Wp, T>) {
		$( self.$element += other.$element );+
	    }
	}

	impl<Wp, T> AddAssign<T> for $self_ty<Wp, T>
	where
	    T: FloatComponent + AddAssign,
	    Wp: WhitePoint,
	{
	    fn add_assign(&mut self, c: T) {
		$( self.$element += c );+
	    }
	}
    }
}

/// Implement `Sub` and `SubAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
#[macro_export]
macro_rules! impl_color_sub {
    ($self_ty: ident , [$($element: ident),+], $phantom: ident) => {

	impl<Wp, T> Sub<$self_ty<Wp, T>> for $self_ty<Wp, T>
	where
	    T: FloatComponent,
	    Wp: WhitePoint,
	{
	    type Output = $self_ty<Wp, T>;

	    fn sub(self, other: $self_ty<Wp, T>) -> Self::Output {
		$self_ty {
		    $( $element: self.$element - other.$element ),+,
		    $phantom: PhantomData,
		}
	    }
	}

	impl<Wp, T> Sub<T> for $self_ty<Wp, T>
	where
	    T: FloatComponent,
	    Wp: WhitePoint,
	{
	    type Output = $self_ty<Wp, T>;

	    fn sub(self, c: T) -> Self::Output {
		$self_ty {
		    $( $element: self.$element - c ),+,
		    $phantom: PhantomData,
		}
	    }
	}

	impl<Wp, T> SubAssign<$self_ty<Wp, T>> for $self_ty<Wp, T>
	where
	    T: FloatComponent + SubAssign,
	    Wp: WhitePoint,
	{
	    fn sub_assign(&mut self, other: $self_ty<Wp, T>) {
		$( self.$element -= other.$element; )+
	    }
	}

	impl<Wp, T> SubAssign<T> for $self_ty<Wp, T>
	where
	    T: FloatComponent + SubAssign,
	    Wp: WhitePoint,
	{
	    fn sub_assign(&mut self, c: T) {
		$( self.$element -= c; )+
	    }
	}
    }
}
