use std::io;
use rand::Rng;
use std::borrow::BorrowMut;

struct Hero {
    name: String,
    health: i32,
    maxHealth: i32,
    xp: u32,
    weapon: String,
    weaponDMG: i32,
}

struct BasicMonster {
    name: String,
    health: i32,
    weapon: String,
    weaponDMG: i32,
}

struct ToughMonster {
    name: String,
    health: i32,
    weapon: String,
    weaponDMG: i32,
}
struct BossMonster {
    name: String,
    health: i32,
    weapon: String,
    weaponDMG: i32,
}

impl BossMonster {
    fn bigDamage(&self) -> i32 {
        rand::thread_rng().gen_range(30,51)
    }
}

struct Town {
    name: String,
}

fn main() {
    // Set up Hero and game base
    let mut usrInput = String::new();

    println!("Welcome to Prevail");
    println!("Please enter your character's name: ");

    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read line");

    let mut hero = Hero {
        name,
        health: 120,
        maxHealth: 120,
        xp: 0,
        weapon: String::from("Basic Sword"),
        weaponDMG: 20,
    };

    let mut bazzel = BossMonster {
        name: "Ba'zzel".parse().unwrap(),
        health: 500,
        weapon: String::from("Void Magic"),
        weaponDMG: 60,
    };


    // Start of game, in town, asked to find the dark lord Ba'zzel and bring peace to the land

    // 3 main areas
    // 1. Woods
    // 2. Mountains
    // 3. Abyss Temple

    // 3 main weapons to gather + starting sword
    // 0. Starting sword, 20 dmg
    // 1. War axe, 30 dmg
    // 2. Greatsword, 50 dmg
    // 3. Warhammer, 60 dmg

    println!("Welcome to Issendra, young hero!\n If it's a quest you're looking for, you've come to
                the right town.\n\
                There's been monsters about seemingly coming from from the the Black Temple atop the Drake Mountain.\n\
                If you can find out why these monsters are appearing, we'll be forever in your debt.\n\
                Not to mention the rewards along the way.");

    println!("Issendra is a large town on the edge of the woods, from here you can start your quest to find monsters[1]. Or quit[0]");
    println!("\n-----------------------------------------------------------------------------------------------------------------------\n");
    println!("From here on out you will enter your commands using numbers only. The last option will be to quit.\n\
                If you quit, your progress will be lost and you will have to start over again. Be careful, young hero.");

    while usrInput != 0.to_string() {
        //Forest loop
        for mut number in 0..21 {
            println!("1. Explore forest\n\
                  2. Rest\n
                  0. Quit");

            // Read action in and change it to an integer. Rust feature
            io::stdin().read_line(&mut usrInput).expect("failed to read line");
            let usrInput: i8 = match usrInput.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };


            match usrInput {
                1 => exploreForest(number, hero.borrow_mut()),
                2 => rest(hero.borrow_mut()),
                0 => break,
                _ => continue
            }

            //After 20 rounds through the forest, the mountain range comes into view and your journey continues
            if number >= 20 {
                println!("You have found your way out of the forest. Continue onwards to the Drake Mountain range to find the temple.");
                break;
            } else if hero.health < 0 {
                break;
            } else {
                continue;
            }
        };

        //Mountain loop
        for mut number in 0..31 {
            println!("1. Explore mountains\n\
                  2. Rest\n
                  0. Quit");

            // Read action in and change it to an integer. Rust feature
            io::stdin().read_line(&mut usrInput).expect("failed to read line");
            let usrInput: i8 = match usrInput.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            match usrInput {
                1 => exploreMountain(number, hero.borrow_mut()),
                2 => rest(hero.borrow_mut()),
                0 => break,
                _ => continue
            }

            //After 30 rounds through the mountain range, the temple finally comes into view. Here is your final challenge
            if number >= 30 {
                println!("You have found your way out of the mountain range.\n\
                      Your final battles await you. Good luck.");
                break;
            } else if hero.health < 0 {
                break;
            } else {
                continue;
            }
        };

        //Temple loop
        for mut number in 0..6 {
            println!("1. Explore Temple\n\
                  2. Rest\n
                  0. Quit");

            // Read action in and change it to an integer. Rust feature
            io::stdin().read_line(&mut usrInput).expect("failed to read line");
            let usrInput: i8 = match usrInput.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            match usrInput {
                1 => exploreTemple(number, hero.borrow_mut()),
                2 => rest(hero.borrow_mut()),
                0 => break,
                _ => continue
            }

            //After 5 rounds through the Temple, you enter Ba'zzel's court
            if number >= 5 {
                println!("You have found Ba'zzel. Defeat him to save the land.");
                break;
            } else if hero.health < 0 {
                break;
            } else {
                continue;
            }
        };

        //boss fight
        bossMnstrFight(hero.borrow_mut(), bazzel.borrow_mut());
    }

}


// Monster set up functions

fn basicMonsterSetUp(name: String, weapon: String) -> BasicMonster {
    let hp = rand::thread_rng().gen_range(30,51);
    BasicMonster {
        name,
        weapon,
        weaponDMG: 15,
        health: hp,
    }
}

fn toughMonsterSetUp(name: String, weapon: String) -> ToughMonster {
    let hp = rand::thread_rng().gen_range(50,71);
    ToughMonster {
        name,
        weapon,
        weaponDMG: 25,
        health: hp,
    }
}

// Town creator
fn townSetUp(name: String) -> Town {
    Town {
        name
    }
}

//Explore Functions
fn exploreForest(count: i8, hero: &mut Hero) {
    println!("You progress through the forest.");
    let monsterChance = rand::thread_rng().gen_range(1,11);
    if count == 15 {
        println!("You have found the war axe!! It's better than your sword, so you equip it.");
        hero.weapon = "WarAxe".parse().unwrap();
        hero.weaponDMG = 30;
    } else if monsterChance < 8 {
        basicMnstrFight(hero, "Ent".parse().unwrap(), "Vines".parse().unwrap());
    } else {
        toughMnstrFight(hero, "Dryad".parse().unwrap(), "ForestMagic".parse().unwrap())
    }
}

fn exploreMountain(count: i8, hero: &mut Hero) {
    println!("You progress through the mountains.");
    let monsterChance = rand::thread_rng().gen_range(1,11);
    if count == 20 {
        println!("You have found the greatsword!! It's better than your axe, so you equip it.");
        hero.weapon = "WarAxe".parse().unwrap();
        hero.weaponDMG = 50;
    } else if monsterChance < 5 {
        basicMnstrFight(hero, "Whelp".parse().unwrap(), "Claw".parse().unwrap());
    } else {
        toughMnstrFight(hero, "Drake".parse().unwrap(), "FireBreath".parse().unwrap())
    }
}

fn exploreTemple(count: i8, hero: &mut Hero) {
    println!("You progress through the Temple.");

    if count == 3 {
        println!("You have found the warhammer!! It's better than your greatsword, so you equip it.");
        hero.weapon = "WarAxe".parse().unwrap();
        hero.weaponDMG = 60;
    } else {
        toughMnstrFight(hero, "Minion".parse().unwrap(), "DarkMagic".parse().unwrap())
    }
}

// Combat functions
fn basicMnstrFight(hero: &mut Hero, name: String, weapon: String) {
    let mut monster = basicMonsterSetUp(name, weapon);

    println!("You fight a {}", monster.name);

    loop {
      if monster.health > 0 && hero.health > 0{
          println!("You swing at the {}!\nYou deal {} damage!", monster.name, hero.weaponDMG);
          monster.health -= hero.weaponDMG;
          println!("The {} fights back!\nTake {} damage!", monster.name, monster.weaponDMG);
          hero.health -= monster.weaponDMG;
      }  else if  monster.health < 0 && hero.health > 0 {
          println!("You have defeated the monster!! Gain 5 xp.");
          hero.xp += 5;
          heroLevel(hero);
          break;
      } else {
          println!("You have died, try again.");
          break;
      }
    };
}

fn toughMnstrFight(hero: &mut Hero, name: String, weapon: String) {
    let mut monster = basicMonsterSetUp(name, weapon);

    println!("You fight a {}", monster.name);

    loop {
        if monster.health > 0 && hero.health > 0{
            println!("You swing at the {}!\nYou deal {} damage!", monster.name, hero.weaponDMG);
            monster.health -= hero.weaponDMG;
            println!("The {} fights back!\nTake {} damage!", monster.name, monster.weaponDMG);
            hero.health -= monster.weaponDMG;
        }  else if  monster.health < 0 {
            println!("You have defeated the monster!! Gain 10 xp.");
            hero.xp += 10;
            heroLevel(hero);
            break;
        } else {
            println!("You have died, try again.");
            break;
        }
    };
}

fn bossMnstrFight(hero: &mut Hero, bigBad: &mut BossMonster) {

    loop {
        if bigBad.health > 0 && hero.health > 0{
            println!("You swing at the {}!\nYou deal {} damage!", bigBad.name, hero.weaponDMG);
            bigBad.health -= hero.weaponDMG;
            println!("The {} fights back!\nTake {} damage!", bigBad.name, bigBad.weaponDMG);
            hero.health -= bigBad.weaponDMG;
        }  else if  bigBad.health < 0 {
            println!("You have defeated Ba'zzel!!!!!");
            break;
        } else {
            println!("You have died, try again.");
            break;
        }
    };
}

// Rest function
fn rest(hero: &mut Hero) {
    println!("You rest up. Some wounds heal and your strength returns.");
    if hero.health == hero.maxHealth {
        println!("Your health is full");
    } else if hero.maxHealth - hero.health < 20 {
        hero.health = hero.maxHealth;
    } else {
        hero.health += 20;
    }
}

//Level check and level up function
fn heroLevel(hero: &mut Hero) {
    if hero.xp == 20 {
        println!("You leveled up!!");
        hero.maxHealth += 20;
        hero.health = hero.maxHealth;
    }
}






