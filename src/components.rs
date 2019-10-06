//! Components and useful data structures
use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
    ecs::world::Index,
};

/// Actions that have a non immediate effect
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Action {
    Charisma,
    //Magic,
    //Power,
    Restart,
    Help,
}

/// state between gameplay and help screen
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentState {
    Intertext,
    Gameplay,
}

/// player stats
#[derive(Debug)]
pub struct Player {
    pub strength: u32,
    pub magic: u32,
    pub charisma: u32,
    pub gold: u32,
    pub action: Option<Action>,
    pub current_state: CurrentState,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            strength: 100,
            magic: 5,
            charisma: 5,
            gold: 100,
            action: None,
            current_state: CurrentState::Gameplay,
        }
    }

}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

/// Empty cell type, with special encounters 
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CellType {
    Empty,
    Fountain,
    Armourer,
    Magician,
    Gold,
}

/// the zone
#[derive(Debug)]
pub struct Zone {
    /// current zone number
    pub current: i32,
    /// target zone
    pub target: i32,
    /// status status
    pub cells: [[i32; 20]; 20],
    /// current cell
    pub cell: (usize,usize),
    /// current cell type
    pub current_type: CellType,
    /// inhabitants cell position
    pub inhabitants: Vec<(usize,usize)>,
    /// shield entities ID
    pub shields: Vec<Index>,
    /// target wizard entity ID
    pub wizard: Option<Index>,
}

impl Component for Zone {
    type Storage = DenseVecStorage<Self>;
}

/// Mark status text
#[derive(Default)]
pub struct Status;

impl Component for Status {
    type Storage = NullStorage<Self>;
}

/// Mark Encounter text
#[derive(Default)]
pub struct Encounter;

impl Component for Encounter {
    type Storage = NullStorage<Self>;
}

/// Cell component
#[derive(Default)]
pub struct Cell{
    pub position: (usize,usize),
}

impl Component for Cell {
    type Storage = DenseVecStorage<Self>;
}

/// Inhabitant marker component
#[derive(Default)]
pub struct Inhabitant {
}

impl Component for Inhabitant {
    type Storage = NullStorage<Self>;
}

/// Shield position component
#[derive(Default)]
pub struct Shield{
    pub position: (usize,usize),
}

impl Component for Shield {
    type Storage = DenseVecStorage<Self>;
}
