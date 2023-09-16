use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
* 숫자 맞히기 게임
* 1. 컴퓨터가 1~100 사이의 숫자를 랜덤으로 생성한다.
* 2. 플레이어가 숫자를 입력한다.
* 3. 생성한 수와 비교해 작으면 작다고, 크면 크다고 출력한다.
* 4. 플레이어가 예측한 숫자와 컴퓨터가 생성한 숫자가 같으면 게임을 종료한다.
*/

fn main() {
    println!("숫자를 맞추자꾸나");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("정답은 무엇인가?:");
        let mut guess = String::new();

        // 참조 역시 기본적으로는 변경 불가하기 때문에 mut 키워드를 사용해야 한다.
        io::stdin()
            .read_line(&mut guess)
            .expect("뭐지?");

        let guess: u32 = match guess
            .trim()
            .parse()
        {
            Ok(num) => num,
            Err(err) => {
                println!("error: {}", err);
                continue;
            }
        };

        println!("너의 추측: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("너무 작아"),
            Ordering::Greater => println!("너무 커"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}
