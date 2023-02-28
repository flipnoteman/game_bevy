/// States to track the completion of asset loading to prevent the 'Err on None()' error
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AssetStates {
    AssetLoading,
    Next,
}

//ToDo: Decide whether or not to move specific asset loading components to this file (i.e PlayerAssets in game_objects/entities/player.rs)