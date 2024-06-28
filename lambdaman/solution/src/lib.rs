#[allow(non_snake_case)]

pub struct OnDrop<F: Fn()> {
    f: F,
}

impl<F: Fn()> OnDrop<F> {
    #[inline]
    pub fn new(f: F) -> Self {
        OnDrop { f }
    }
}

impl<F: Fn()> Drop for OnDrop<F> {
    #[inline]
    fn drop(&mut self) {
        (self.f)()
    }
}

pub fn bench(id: String) -> OnDrop<impl Fn()> {
    eprintln!("Start({})", id);
    let t = ::std::time::SystemTime::now();
    OnDrop::new(move || {
        let d = t.elapsed().unwrap();
        let s = d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9;
        eprintln!("Time({}) = {:.3}", id, s);
    })
}

#[macro_export]
macro_rules! bench {
	([$name:expr]$($e: tt)*) => {
		let b = $crate::bench($name.to_owned());
		$($e)*
		drop(b);
	};
	($($e: tt)*) => {
		let b = $crate::bench(format!("{}:{}", file!(), line!()));
		$($e)*
		drop(b);
	};
}

pub static mut PROFILER: *mut Vec<(&str, &(f64, usize, usize))> = 0 as *mut Vec<_>;

#[macro_export]
macro_rules! profile {
    ($id:ident) => {
        static mut __PROF: (f64, usize, usize) = (0.0, 0, 0);
        unsafe {
            if __PROF.1 == 0 {
                if $crate::PROFILER.is_null() {
                    $crate::PROFILER = Box::into_raw(Box::new(Vec::new()));
                }
                (*$crate::PROFILER).push((stringify!($id), &__PROF));
            }
            if __PROF.2 == 0 {
                let d = ::std::time::SystemTime::now()
                    .duration_since(::std::time::SystemTime::UNIX_EPOCH)
                    .unwrap();
                __PROF.0 -= d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9;
            }
            __PROF.1 += 1;
            __PROF.2 += 1;
        }
        #[allow(unused)]
        let $id = $crate::OnDrop::new(move || unsafe {
            __PROF.2 -= 1;
            if __PROF.2 == 0 {
                let d = ::std::time::SystemTime::now()
                    .duration_since(::std::time::SystemTime::UNIX_EPOCH)
                    .unwrap();
                __PROF.0 += d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9;
            }
        });
    };
}

#[macro_export]
macro_rules! count {
    ($id:ident) => {
        static mut __PROF: (f64, usize, usize) = (0.0, 0, 0);
        unsafe {
            if __PROF.1 == 0 {
                if $crate::PROFILER.is_null() {
                    $crate::PROFILER = Box::into_raw(Box::new(Vec::new()));
                }
                (*$crate::PROFILER).push((stringify!($id), &__PROF));
            }
            __PROF.1 += 1;
        }
    };
}

pub fn write_profile() {
    let mut ps: Vec<_> = unsafe {
        if PROFILER.is_null() {
            return;
        }
        (*PROFILER).clone()
    };
    ps.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());
    eprintln!("########## Profile ##########");
    for (id, &(mut t, c, depth)) in ps {
        if depth > 0 {
            let d = ::std::time::SystemTime::now()
                .duration_since(::std::time::SystemTime::UNIX_EPOCH)
                .unwrap();
            t += d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9;
        }
        eprintln!("{}:\t{:.3}\t{}", id, t, c);
    }
    eprintln!("#############################");
}

#[macro_export]
macro_rules! optuna {
	($($p:ident: $t:tt = suggest($def:expr, $($a:tt),*)),* $(,)*) => {
		#[derive(Debug, Clone)]
		struct Param {
			$($p: $t,)*
		}
		lazy_static::lazy_static! {
			static ref PARAM: Param = {
				$(let $p = std::env::var(stringify!($p)).map(|s| s.parse().expect(concat!("failed to parse ", stringify!($p)))).unwrap_or($def);)*
				Param { $( $p ),* }
			};
		}
		impl Param {
			fn optuna_str() -> String {
				let mut list = vec![];
				$(list.push($crate::optuna!(# $p, $t, $($a),*));)*
				let mut s = "def setup(trial):\n".to_owned();
				for (t, _) in &list {
					s += "\t";
					s += &t;
					s += "\n";
				}
				s += "\tenv = {";
				for (i, (_, t)) in list.iter().enumerate() {
					if i > 0 {
						s += ", ";
					}
					s += &t;
				}
				s += "}\n\treturn env";
				s
			}
		}
	};
	(# $p:ident, f64, $min:expr, $max:expr) => {
		(format!("{} = trial.suggest_float(\"{}\", {}, {})", stringify!($p), stringify!($p), $min, $max), format!("\"{}\": str({})", stringify!($p), stringify!($p)))
	};
	(# $p:ident, usize, $min:expr, $max:expr) => {
		(format!("{} = trial.suggest_int(\"{}\", {}, {})", stringify!($p), stringify!($p), $min, $max), format!("\"{}\": str({})", stringify!($p), stringify!($p)))
	};
	(# $p:ident, f64, $min:expr, $max:expr, log) => {
		(format!("{} = trial.suggest_float(\"{}\", {}, {}, log=True)", stringify!($p), stringify!($p), $min, $max), format!("\"{}\": str({})", stringify!($p), stringify!($p)))
	};
	(# $p:ident, f64, $min:expr, $max:expr, $step:expr) => {
		(format!("{} = trial.suggest_float(\"{}\", {}, {}, {})", stringify!($p), stringify!($p), $min, $max, $step), format!("\"{}\": str({})", stringify!($p), stringify!($p)))
	};
}
