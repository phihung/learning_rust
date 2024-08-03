// https://leetcode.com/problems/filling-bookcase-shelves

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        // Self::dynamic_programming(books, shelf_width)
        Self::memorized_backtracking(books, shelf_width)
    }

    // result[i] = min({ max(h[(j+1)..i]) + result[j] | sum(w[(j+1)..i)] <= width })
    // Cost i books = min_j { (cost j first books) + (height of last i - j books in the same row) }
    pub fn dynamic_programming(books: Vec<Vec<i32>>, width: i32) -> i32 {
        let n = books.len();
        let mut memory: Vec<_> = vec![i32::MAX; n + 1];
        memory[0] = 0;
        for i in 1..=n {
            let (mut w, mut h) = (0, 0);
            for j in (0..i).rev() {
                w += books[j][0];
                h = h.max(books[j][1]);
                if w > width {
                    break;
                }
                memory[i] = memory[i].min(h + memory[j]);
            }
        }
        *memory.last().unwrap()
    }

    // 0ms / 2.3Mb
    pub fn memorized_backtracking(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        fn backtracking(
            width: i32,
            books: &[Vec<i32>],
            (mut row_w, mut row_h): (i32, i32),
            memory: &mut [i32],
        ) -> i32 {
            let n = books.len();
            if n == 0 {
                return row_h;
            }
            if memory[n - 1] < i32::MAX {
                return memory[n - 1];
            }
            let mut best = i32::MAX;
            let mut new_line = false;
            for i in 0..n {
                let (w, h) = (books[i][0], books[i][1]);
                new_line = row_w + w > width;
                if new_line || h > row_h && row_h > 0 {
                    best = best.min(row_h + backtracking(width, &books[(i + 1)..], (w, h), memory));
                }
                if new_line {
                    break;
                }
                row_h = row_h.max(h);
                row_w += w;
            }
            if !new_line {
                best = best.min(row_h)
            }
            memory[n - 1] = best;
            best
        }

        let mut memory = vec![i32::MAX; books.len()];
        backtracking(shelf_width, &books, (0, 0), &mut memory)
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        let func = |books: &[[i32; 2]], w: i32| {
            Solution::min_height_shelves(books.iter().map(|x| x.to_vec()).collect(), w)
        };
        run_test(func)
    }

    #[test]
    fn test_dynamic_programming() {
        let func = |books: &[[i32; 2]], w: i32| {
            Solution::dynamic_programming(books.iter().map(|x| x.to_vec()).collect(), w)
        };
        run_test(func)
    }

    #[test]
    fn test_backtracking() {
        let func = |books: &[[i32; 2]], w: i32| {
            Solution::memorized_backtracking(books.iter().map(|x| x.to_vec()).collect(), w)
        };
        run_test(func)
    }

    fn run_test(func: impl Fn(&[[i32; 2]], i32) -> i32) {
        assert_eq!(func(&[[1, 1]], 2), 1);
        assert_eq!(func(&[[1, 1], [1, 2], [1, 3], [1, 3]], 3), 4);
        assert_eq!(func(&[[1, 1], [1, 2], [1, 2], [1, 2]], 3), 3);
        assert_eq!(func(&[[1, 1], [1, 2], [1, 2], [1, 2], [1, 2]], 3), 4);
        assert_eq!(
            func(&[[1, 1], [1, 2], [1, 2], [1, 2], [1, 2], [2, 2]], 3),
            5
        );
        assert_eq!(func(&[[1, 1], [1, 2]], 2), 2);
        assert_eq!(func(&[[1, 1], [2, 2]], 2), 3);
        assert_eq!(
            func(&[[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]], 4),
            6
        );
        assert_eq!(func(&[[1, 3], [2, 4], [3, 2]], 6), 4);

        #[rustfmt::skip]
        let a = [[11,83],[170,4],[93,80],[155,163],[134,118],[75,14],[122,192],[123,154],[187,29],[160,64],[170,152],[113,179],[60,102],[28,187],[59,95],[187,97],[49,193],[67,126],[75,45],[130,160],[4,102],[116,171],[43,170],[96,188],[54,15],[167,183],[58,158],[59,55],[148,183],[89,95],[90,113],[51,49],[91,28],[172,103],[173,3],[131,78],[11,199],[77,200],[58,65],[77,30],[157,58],[18,194],[101,148],[22,197],[76,181],[21,176],[50,45],[80,174],[116,198],[138,9],[58,125],[163,102],[133,175],[21,39],[141,156],[34,185],[14,113],[11,34],[35,184],[16,132],[78,147],[85,170],[32,149],[46,94],[196,3],[155,90],[9,114],[117,119],[17,157],[94,178],[53,55],[103,142],[70,121],[9,141],[16,170],[92,137],[157,30],[94,82],[144,149],[128,160],[8,147],[153,198],[12,22],[140,68],[64,172],[86,63],[66,158],[23,15],[120,99],[27,165],[79,174],[46,19],[60,98],[160,172],[128,184],[63,172],[135,54],[40,4],[102,171],[29,125],[81,9],[111,197],[16,90],[22,150],[168,126],[187,61],[47,190],[54,110],[106,102],[55,47],[117,134],[33,107],[2,10],[18,62],[109,188],[113,37],[59,159],[120,175],[17,147],[112,195],[177,53],[148,173],[29,105],[196,32],[123,51],[29,19],[161,178],[148,2],[70,124],[126,9],[105,87],[41,121],[147,10],[78,167],[91,197],[22,98],[73,33],[148,194],[166,64],[33,138],[139,158],[160,19],[140,27],[103,109],[88,16],[99,181],[2,140],[50,188],[200,77],[73,84],[159,130],[115,199],[152,79],[1,172],[124,136],[117,138],[158,86],[193,150],[56,57],[150,133],[52,186],[21,145],[127,97],[108,110],[174,44],[199,169],[139,200],[66,48],[52,190],[27,86],[142,191],[191,79],[126,114],[125,100],[176,95],[104,79],[146,189],[144,78],[52,106],[74,74],[163,128],[34,181],[20,178],[15,107],[105,8],[66,142],[39,126],[95,59],[164,69],[138,18],[110,145],[128,200],[149,150],[149,93],[145,140],[90,170],[81,127],[57,151],[167,127],[95,89]];
        assert_eq!(func(&a, 200), 15672);
    }
}
