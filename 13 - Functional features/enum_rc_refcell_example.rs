use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

fn main() {
    // Usando Rc<RefCell<T>> para compartilhamento
    let game_state = Rc::new(RefCell::new(GameState::Menu));
    
    println!("Estado inicial: {:?}", game_state.borrow());
    
    // Closure que modifica o estado
    let state_clone = Rc::clone(&game_state);
    let start_game = move || {
        let mut state = state_clone.borrow_mut();
        *state = GameState::Playing;
        println!("Jogo iniciado! Estado atual: {:?}", *state);
    };
    
    // Outra closure para pausar
    let state_clone2 = Rc::clone(&game_state);
    let pause_game = move || {
        let mut state = state_clone2.borrow_mut();
        match *state {
            GameState::Playing => {
                *state = GameState::Paused;
                println!("Jogo pausado!");
            },
            _ => println!("Não é possível pausar neste estado: {:?}", *state),
        }
    };
    
    start_game();
    println!("Estado após iniciar: {:?}", game_state.borrow());
    
    pause_game();
    println!("Estado final: {:?}", game_state.borrow());
}
