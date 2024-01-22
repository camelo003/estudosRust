use std::io::stdin;

#[derive(Debug)]
enum VisitorDiet {
		Meat,
		Oni,
		Vegan,
		Allergic {food: String},
}

#[derive(Debug)]
struct Visitor {
		name: String,
		greeting: String,
		diet: VisitorDiet,
}

impl Visitor {
		fn new(name: &str, greeting: &str, diet: VisitorDiet) -> Self {
				Self {
						name: name.to_lowercase(),
						greeting: greeting.to_lowercase(),
						diet,
				}
		}

		fn greet_visitor(&self) {
				println!("\n[SEGURANÇA]\n{}", self.greeting);
				match &self.diet {
						VisitorDiet::Meat => println!("não deixe de provar a picanha."),
						VisitorDiet::Oni => println!("peça uma carne com uma salada."),
						VisitorDiet::Vegan => println!("servimos leite de soja."),
						VisitorDiet::Allergic { food } => {
								println!("Vou pedir pra não usarem {}.", food.trim());
						}
				}
		}
}

fn what_is_your_name() -> String {
		let mut your_name = String::new();
		println!("[VOCÊ]");
		stdin().read_line(&mut your_name).expect("\nerro ao ler resposta.\n");
		your_name.trim().to_lowercase()
}

fn first_came() -> Visitor {
		println!("\n[SEGURANÇA]\nnunca te vi. qual é o seu nome mesmo?");
		println!("\n[VOCÊ]");
		let mut new_name = String::new();
		stdin()
				.read_line(&mut new_name)
				.expect("erro ao ler resposta.");
		new_name = new_name.trim().to_lowercase();
		println!("\n[SEGURANÇA]\ne como gostaria de ser recebido?");
		println!("\n[VOCÊ]");
		let mut new_greeting = String::new();
		stdin()
				.read_line(&mut new_greeting)
				.expect("erro ao ler resposta.");
		new_greeting = new_greeting.trim().to_lowercase();
		let mut diet_answer = String::new();
		let new_diet : VisitorDiet;
		loop {
				println!("\n[SEGURANÇA]\npor último, como define sua dieta? responda com:");
				println!("[1] para carnívora;");
				println!("[2] para onívora;");
				println!("[3] para vegana;");
				println!("[4] se tiver alguma alergia.");
				println!("\n[VOCÊ]");
				stdin().read_line(&mut diet_answer).expect("erro ao ler resposta.");
				match diet_answer.trim() {
						"1" => {new_diet = VisitorDiet::Meat; break;},
						"2" => {new_diet = VisitorDiet::Oni; break;},
						"3" => {new_diet = VisitorDiet::Vegan; break;},
						"4" => {
								let mut new_food = String::new();
								println!("\n[SEGURANÇA]\ntem alergia de que!?.");
								println!("\n[VOCÊ]");
								stdin().read_line(&mut new_food)
										.expect("erro ao ler resposta.");
								new_diet = VisitorDiet::Allergic{food: new_food};
								break;
						},
						_ => println!("\n[SEGURANÇA]\nnão entendi."),
				}
		}
		println!("\n[SEGURANÇA]\nobrigado! na próxima você pode entrar!");
		Visitor::new(&new_name, &new_greeting, new_diet)
}

fn main() {
		let mut name;
		let mut know_visitor;
		let mut visitors = vec![
				Visitor::new("gabriel",
										 "divirta-se, gabriel!",
										 VisitorDiet::Meat),
				Visitor::new("fernando",
										 "fernando, seu leite está na geladeira.",
										 VisitorDiet::Vegan),
				Visitor::new("marcos",
										 "marcos!? quem te convidou?",
										 VisitorDiet::Oni),
				Visitor::new("edu",
										 "fala, duds! hehe...",
										 VisitorDiet::Allergic {food: String::from("camarão")}),
				];

		loop {
				println!("[SEGURANÇA]\nolá! qual é o seu nome? (vazio p/ encerrar)\n");
				name = what_is_your_name();
				know_visitor = visitors.iter().find(|visitor| visitor.name == name);

				match know_visitor {
						Some(visitor) => visitor.greet_visitor(),
						None => {
								if name.is_empty() {
										break;
								}else{
										visitors.push(first_came());
								}
						}
				}
				println!("\n----- ----- [PRÓX. DA FILA] ----- -----\n");
		}
		println!("\nlista de convidados ao fim da noite:\n");
		println!("{:?}", visitors);
}
