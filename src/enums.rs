use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TextField {
    HelpText,
    EnemiesCounter,
    RoundCounter,
    PointsCounter,
    DamageModifier,
    RangeModifier,
    FireRateModifier,
    Heal,
    Health,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Modifier {
    Damage,
    Range,
    FireRate,
    Heal,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Enemy,
    Bullet,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum EnemyType {
    Basic,
    Tank,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BulletType {
    Basic,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum WeaponType {
    Pistol,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveBehaviour {
    PointedToPlayer,
    AvoidEnemies,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum AppState {
    MainMenu,
    Playing,
    GameOver,
    Paused,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ButtonID {
    NewGame,
    Exit,
    MainMenu,
}

