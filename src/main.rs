//match es un constructo que permite comparar un valor y una serie de patterns para executar cierto codigo
//en funcion del pattern que hace match
//los patterns pueden estar conformados de literales, variables, wildcards, etc.
//la expresion que evalua if debe ser boolean, pero match puede devolver cualquier otro tipo
fn main() {
    //match tiene la misma logica de first arm, solo el primer brazo que hace match se ejecuta
    //enum que lista variantes de monedas
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    //enum de estados que se incluye en el enum de Coin
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Hawai,
        California,
    }

    //funcion que recibe un enum de coin y devuelve su valor en cents
    fn value_in_cents(coin: Coin) -> u8{
        //evalua el enum
        match coin {
            //entrara a cada brazo dependiendo de la variante de coin
            Coin::Penny => {
                println!("centavo de la suerte!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            //se pueden obtener valores con las expresiones
            //la variable state toma el valor del estado que tiene el enum enviado
            Coin::Quarter(state) => {
                println!("moneda perteneciente al estado de: {:?}", state);
                25
            },
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alabama));

    //incluso podemos acceder al parametro T

    //el tipo va sobre T y todo el enum se almacena en x
    //devuelve un nuevo enum, None en caso de que no exista valor y si existe le suma 1 a T
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let two = plus_one(Some(1));
    let another_none = plus_one(None);

    dbg!(two);
    dbg!(another_none);

    //Match es un constructo exhaustivo, lo que significa que se deben de definir todos los posibles valores
    //incluso la falta del mismo, incluir Option sin None, da un error de compilacion

    //catchall y _ placeholder

    //cuando solo tenemos interes en ciertas posibilidades y en todas las restantes queremos un comportamiento
    //oidemos usar el placeholder _ para indicar que todos los valores restantes deben caer en esa opcion y no interesa cual o almacenarlo
    //si queremos almacenarlo podemos usar una variable como other
    //el catchall siempre va al final, para que se consideren las opciones exclusivas
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //para cualquier otro valor no se ejecuta ninguna accion, es lo que indicamos con la tupla unidad
        _ => (),
        //other => move_player(other), en caso de cualquier valor que no sea 3 y 7, donde ademas queremos obtener ese valor

    }

    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}

    //if let
    //cuando queremos obtener el primer brazo del match y el uso de _ queda como boilerplate, tenemos a la mano if let

    //codigo con match
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("max_config is {}", max),
        _ => (),
    }

    //usando if let como sugar syntax al costo de exhaustive checking

    if let Some(max) = config_max { //cuando el enum coincide con Some(max), podemos usar max en el cuerpo del if
        println!("max_config is {}", max);
    }

}
