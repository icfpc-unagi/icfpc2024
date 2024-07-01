#![allow(non_snake_case)]

use fixedbitset::FixedBitSet;
use itertools::Itertools;
use nalgebra::coordinates::X;
use num_traits::real;
use rustc_hash::{FxHashMap, FxHashSet};
use solution::*;

const TL: f64 = 1.0 * 10.0;
const V_PENALITY: i64 = 1;
const TARGET_T: i64 = 1;

// 訪問済みの頂点数で区切ってビームサーチをする
// 訪問順の多様性を重視

fn main() {
    get_time();
    let input = read_input();
    let best = if let Ok(best) = std::env::var("BEST") {
        read_output(&best)
    } else {
        vec![]
    };
    let mut best_state = State {
        visited: FixedBitSet::with_capacity(input.ps.len()),
        p: (0, 0),
        vy_min: 0,
        vy_max: 0,
        vx_min: 0,
        vx_max: 0,
        t: 0,
        id: !0,
    };
    let mut beam = vec![State {
        visited: FixedBitSet::with_capacity(input.ps.len()),
        p: (0, 0),
        vy_min: 0,
        vy_max: 0,
        vx_min: 0,
        vx_max: 0,
        t: 0,
        id: !0,
    }];

    let predp_time = 400;
    let mut pre_dpmin: Vec<Vec<i32>> = vec![vec![]; predp_time];
    let mut pre_dpmax: Vec<Vec<i32>> = vec![vec![]; predp_time];
    for t in 0..predp_time {
        let max_move = (t + 0) * (t + 1) / 2;
        pre_dpmin[t] = vec![999999; max_move + 1];
        pre_dpmax[t] = vec![-999999; max_move + 1];
        if t == 0 {
            pre_dpmin[t][0] = 0;
            pre_dpmax[t][0] = 0;
        } else {
            for i in 0..pre_dpmin[t - 1].len() {
                let minv = pre_dpmin[t - 1][i];
                let maxv = pre_dpmax[t - 1][i];
                if minv == 999999 {
                    continue;
                }
                for v in minv..=maxv {
                    for add_v in -1..=1 {
                        let mut next_v = v + add_v;
                        let mut next_p = i as i32 + next_v;
                        if next_p < 0 {
                            next_p = -next_p;
                            next_v = -next_v;
                        }
                        pre_dpmin[t][next_p as usize] = pre_dpmin[t][next_p as usize].min(next_v);
                        pre_dpmax[t][next_p as usize] = pre_dpmax[t][next_p as usize].max(next_v);
                    }
                }
            }
        }
    }

    //beam.push(best_state.clone());

    let mut trace: Trace<((i64, i64), i64, (i64, i64, i64, i64))> = Trace::new();
    for k in 0..input.ps.len() {
        let mut next = vec![];
        let mut w = 0;
        for state in beam {
            w += 1;
            let target = {
                let mut list = BoundedSortedList::new(10);
                for i in 0..input.ps.len() {
                    if !state.visited[i] {
                        let mut tx1 = input.ps[i].0 - state.p.0 - state.vx_min * TARGET_T;
                        let mut tx2 = input.ps[i].0 - state.p.0 - state.vx_max * TARGET_T;
                        let mut kx = tx1.abs().min(tx2.abs());
                        if tx1 * tx2 <= 2 {
                            kx = 0;
                        }

                        let mut ty1 = input.ps[i].1 - state.p.1 - state.vy_min * TARGET_T;
                        let mut ty2 = input.ps[i].1 - state.p.1 - state.vy_max * TARGET_T;
                        let mut ky = ty1.abs().min(ty2.abs());
                        if ty1 * ty2 <= 2 {
                            ky = 0;
                        }

                        list.insert((kx + ky), i);
                    }
                }
                list.list().into_iter().map(|(_, i)| i).collect_vec()
            };

            for i in target {
                let mut T = 0;
                loop {
                    T += 1;
                    if T >= predp_time as i64 {
                        break;
                    }

                    let dx = input.ps[i].0;
                    let minx = state.p.0 + state.vx_min * T - T * (T + 1) / 2;
                    let maxx = state.p.0 + state.vx_max * T + T * (T + 1) / 2;
                    if minx > dx || maxx < dx {
                        continue;
                    }

                    let dy = input.ps[i].1;
                    let miny = state.p.1 + state.vy_min * T - T * (T + 1) / 2;
                    let maxy = state.p.1 + state.vy_max * T + T * (T + 1) / 2;
                    if miny > dy || maxy < dy {
                        continue;
                    }

                    break;
                }

                if w == -1 {
                    println!(
                        "{} {} {}  {} {} {} {} {} {}",
                        input.ps[i].0,
                        input.ps[i].1,
                        T,
                        state.p.0,
                        state.p.1,
                        state.vy_min,
                        state.vy_max,
                        state.vx_min,
                        state.vx_max,
                    );
                }

                T -= 1;

                //print!("fT: {} ", T);
                let mut lcount = 0;

                //let mut used = FxHashSet::default();
                while lcount < 3 {
                    T += 1;
                    lcount += 1;
                    //eprint!("nowT: {} ", T);
                    if T >= predp_time as i64 {
                        break;
                    }
                    let dx = input.ps[i].0 - state.p.0;
                    let mut next_minvx = 999999;
                    let mut next_maxvx = -999999;
                    for vx in state.vx_min..=state.vx_max {
                        let real_dx = dx - vx * T;
                        let fixdx = real_dx.abs();
                        if fixdx >= pre_dpmin[T as usize].len() as i64 {
                            continue;
                        }
                        let minvx = pre_dpmin[T as usize][fixdx as usize];
                        let maxvx = pre_dpmax[T as usize][fixdx as usize];
                        if real_dx >= 0 {
                            next_minvx = next_minvx.min(minvx + vx as i32);
                            next_maxvx = next_maxvx.max(maxvx + vx as i32);
                        } else {
                            next_minvx = next_minvx.min(vx as i32 - maxvx);
                            next_maxvx = next_maxvx.max(vx as i32 - minvx);
                        }
                    }

                    let dy = input.ps[i].1 - state.p.1;
                    let mut next_minvy = 999999;
                    let mut next_maxvy = -999999;
                    for vy in state.vy_min..=state.vy_max {
                        let real_dy = dy - vy * T;
                        let fixdy = real_dy.abs();
                        if fixdy >= pre_dpmin[T as usize].len() as i64 {
                            continue;
                        }
                        let minvy = pre_dpmin[T as usize][fixdy as usize];
                        let maxvy = pre_dpmax[T as usize][fixdy as usize];
                        if real_dy >= 0 {
                            next_minvy = next_minvy.min(minvy + vy as i32);
                            next_maxvy = next_maxvy.max(maxvy + vy as i32);
                        } else {
                            next_minvy = next_minvy.min(vy as i32 - maxvy);
                            next_maxvy = next_maxvy.max(vy as i32 - minvy);
                        }
                    }
                    if next_minvx == 999999 || next_minvy == 999999 {
                        lcount -= 1;
                        continue;
                    }

                    let mut p = input.ps[i];
                    let mut id = trace.add(
                        (
                            p,
                            T,
                            (
                                next_minvy as i64,
                                next_maxvy as i64,
                                next_minvx as i64,
                                next_maxvx as i64,
                            ),
                        ),
                        state.id,
                    );

                    let mut visited = state.visited.clone();
                    visited.insert(i);
                    next.push(State {
                        visited,
                        p,
                        vy_min: next_minvy as i64,
                        vy_max: next_maxvy as i64,
                        vx_min: next_minvx as i64,
                        vx_max: next_maxvx as i64,
                        t: state.t + T,
                        id: id,
                    });
                }
            }
            if w > 10000 && get_time() > TL * (k + 1) as f64 / input.ps.len() as f64 {
                break;
            }
        }

        if k + 1 == input.ps.len() {
            next.sort_by_key(|s| s.t);
        } else {
            next.sort_by_key(|s| {
                let vx_add = {
                    if s.vx_max * s.vx_min <= 0 {
                        0
                    } else {
                        s.vx_max.abs().min(s.vx_min.abs())
                    }
                };
                let vy_add = {
                    if s.vy_max * s.vy_min <= 0 {
                        0
                    } else {
                        s.vy_max.abs().min(s.vy_min.abs())
                    }
                };

                s.t * 100000000 + 1 * (vx_add + vy_add)
                    - (s.vx_max - s.vx_min + 1) * (s.vy_max - s.vy_min + 1)
            });
        }
        //eprintln!("{}", next.len());

        eprintln!("{} / {}: w = {}, t = {}", k, input.ps.len(), w, next[0].t);
        beam = vec![];
        //let mut used = FxHashSet::default();

        let mut used_map: FxHashMap<((i64, i64), FixedBitSet), (i64, i64, i64, i64)> =
            FxHashMap::default();

        for s in next {
            let h = (s.p, s.visited.clone());
            if used_map.contains_key(&h) {
                if used_map[&h].0 <= s.vx_min
                    && s.vx_max <= used_map[&h].1
                    && used_map[&h].2 <= s.vy_min
                    && s.vy_max <= used_map[&h].3
                {
                    continue;
                } else {
                    let now = used_map[&h];
                    used_map.insert(
                        h,
                        (
                            s.vx_min.min(now.0),
                            s.vx_max.max(now.1),
                            s.vy_min.min(now.2),
                            s.vy_max.max(now.3),
                        ),
                    );
                }
            } else {
                used_map.insert(h, (s.vx_min, s.vx_max, s.vy_min, s.vy_max));
            }
            beam.push(s);
            if beam.len() >= 100000 {
                break;
            }
        }

        /*
        let live = beam
            .iter()
            .map(|s| s.id)
            .chain(vec![best_state.id])
            .collect_vec();
        let ids = trace.compact(&live);
        for s in beam.iter_mut() {
            s.id = if s.id == !0 { !0 } else { ids[s.id] };
        }
        best_state.id = if best_state.id == !0 {
            !0
        } else {
            ids[best_state.id]
        };

        */
    }
    let b = beam.iter().min_by_key(|s| s.t).unwrap();

    eprintln!("start trace");

    let mut output_array = vec![];
    output_array.push(1);

    let mut trace_array = vec![];
    for mv in trace.get(b.id) {
        trace_array.push(mv);
    }

    eprintln!("reverse trace");
    trace_array.reverse();
    trace_array.push(((0, 0), 0, (0, 0, 0, 0)));

    let mut goal_info = trace_array[0].clone();

    for ti in 0..trace_array.len() {
        eprintln!(
            "{} {} {} {} {}",
            trace_array[ti].0 .0,
            trace_array[ti].2 .2,
            trace_array[ti].0 .1,
            trace_array[ti].2 .0,
            trace_array[ti].1
        )
    }

    let mut goal_vx = b.vx_max;
    let mut goal_vy = b.vy_max;
    //eprintln!("first_v {} {}", goal_vx, goal_vy);

    //eprintln!("first pos {} {}", goal_info.0 .0, goal_info.0 .1);

    for ti in 1..trace_array.len() {
        let mv = trace_array[ti].clone();

        let move_time = goal_info.1;

        let start_y = mv.0 .1;
        let goal_y = goal_info.0 .1;

        let mut dy = vec![];

        for start_vy in mv.2 .0..=mv.2 .1 {
            let mut tmpdy = vec![];
            let mut remain_t = move_time;

            let mut tmp_vy = start_vy;
            let mut tmp_y = start_y;

            /*
            eprintln!(
                "starty: {}, start_vy: {}, time: {}, goal_y: {}, goal_vy: {}",
                start_y,
                start_vy, move_time, goal_y, goal_vy
            );
            */

            while remain_t > 0 {
                let mut ok = false;

                for add_vy in -1..=1 {
                    let t2 = tmp_vy + add_vy;
                    let r2 = remain_t - 1;
                    let n2 = tmp_y + t2;

                    let real_dy = goal_y - (n2 + t2 * r2);
                    let fixdy = real_dy.abs();

                    //println!("precheck: {} {}", real_dy, pre_dpmin[r2 as usize].len());

                    if fixdy >= pre_dpmin[r2 as usize].len() as i64 {
                        continue;
                    }
                    //println!("check vy = {}", t2);
                    let minvy = {
                        if real_dy >= 0 {
                            pre_dpmin[r2 as usize][fixdy as usize] + t2 as i32
                        } else {
                            -pre_dpmax[r2 as usize][fixdy as usize] + t2 as i32
                        }
                    };
                    let maxvy = {
                        if real_dy >= 0 {
                            pre_dpmax[move_time as usize][fixdy as usize] + t2 as i32
                        } else {
                            -pre_dpmin[move_time as usize][fixdy as usize] + t2 as i32
                        }
                    };
                    //println!("max:{} min:{}", maxvy, minvy);
                    if minvy as i64 <= goal_vy && goal_vy <= maxvy as i64 {
                        tmpdy.push(add_vy);
                        tmp_vy = t2;
                        remain_t = r2;
                        tmp_y = n2;
                        ok = true;
                        break;
                    }
                }

                if !ok {
                    break;
                }
            }

            if tmpdy.len() == move_time as usize {
                dy = tmpdy;
                for t in &dy {
                    goal_vy -= t;
                }
                //dy.reverse();
                break;
            }
        }

        eprintln!("yok! {} {}", move_time, dy.len());

        let start_x = mv.0 .0;
        let goal_x = goal_info.0 .0;

        let mut dx = vec![];

        for start_vx in mv.2 .2..=mv.2 .3 {
            let mut tmpdx = vec![];
            let mut remain_t = move_time;

            let mut tmp_vx = start_vx;
            let mut tmp_x = start_x;

            while remain_t > 0 {
                let mut ok = false;

                for add_vx in -1..=1 {
                    let t2 = tmp_vx + add_vx;
                    let r2 = remain_t - 1;
                    let n2 = tmp_x + t2;

                    let real_dx = goal_x - (n2 + t2 * r2);
                    let fixdx = real_dx.abs();

                    if fixdx >= pre_dpmin[r2 as usize].len() as i64 {
                        continue;
                    }
                    let minvx = {
                        if real_dx >= 0 {
                            pre_dpmin[r2 as usize][fixdx as usize] + t2 as i32
                        } else {
                            -pre_dpmax[r2 as usize][fixdx as usize] + t2 as i32
                        }
                    };
                    let maxvx = {
                        if real_dx >= 0 {
                            pre_dpmax[move_time as usize][fixdx as usize] + t2 as i32
                        } else {
                            -pre_dpmin[move_time as usize][fixdx as usize] + t2 as i32
                        }
                    };
                    if minvx as i64 <= goal_vx && goal_vx <= maxvx as i64 {
                        tmpdx.push(add_vx);
                        tmp_vx = t2;
                        remain_t = r2;
                        tmp_x = n2;
                        ok = true;
                        break;
                    }
                }

                if !ok {
                    break;
                }
            }

            if tmpdx.len() == move_time as usize {
                dx = tmpdx;
                for t in &dx {
                    goal_vx -= t;
                }
                //dx.reverse();
                break;
            }
        }

        eprintln!("xok! {} {}", move_time, dx.len());

        for ii in 0..dx.len() {
            let i = dx.len() - ii - 1;
            let vx = dx[i];
            let vy = dy[i];
            let mv = (vy + 1) * 3 + (vx + 1) + 1;
            output_array.push(mv);
        }

        goal_info = mv.clone();
    }

    output_array.reverse();

    for i in output_array {
        println!("{}", i);
    }

    eprintln!("Time = {:.3}", get_time());
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    visited: FixedBitSet,
    p: (i64, i64),
    vy_min: i64,
    vy_max: i64,
    vx_min: i64,
    vx_max: i64,
    t: i64,
    id: usize,
}
