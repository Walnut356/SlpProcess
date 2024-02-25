use crate::{
    events::{item_frames::ItemFrames, post_frame::PostFrames, pre_frame::PreFrames},
    game::GameMetadata,
    stats::*,
    Game,
};
use polars::prelude::*;
use ssbm_utils::prelude::*;
use std::ops::Div;

pub fn downcast_u8(series: &Series) -> Result<Vec<Option<u8>>, PolarsError> {
    let chunked = series.u8()?;
    Ok(chunked.to_vec())
}

pub(crate) fn as_vec_static_str<T: Into<&'static str>>(input: Vec<T>) -> Vec<&'static str> {
    input
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<&'static str>>()
}

pub(crate) fn as_vec_arrow(input: Vec<StickRegion>) -> Vec<&'static str> {
    input.into_iter().map(|x| x.to_utf_arrow()).collect()
}

impl Game {
    pub fn summary_df(&self) -> DataFrame {
        // I fucking hate time libraries so much. All of them have ergonomics like this.
        let date = self.date();
        // .to_offset(time::UtcOffset::current_local_offset().unwrap());
        let offset = time::UtcOffset::current_local_offset().unwrap();

        let v_s = vec![
            // changing the offset doesn't actually change the underlying value so we have to do it manually =)
            Series::new(
                "Datetime",
                &[AnyValue::Datetime(
                    // manually coerce offset into seconds and seconds into milliseconds
                    (date.unix_timestamp() + (offset.whole_hours() as i64 * 3600)) * 1000,
                    TimeUnit::Milliseconds,
                    &None,
                )],
            ),
            Series::new(
                "Duration",
                &[AnyValue::Duration(
                    self.duration().as_millis() as i64,
                    TimeUnit::Milliseconds,
                )],
            ),
            Series::new("MatchType", &[Into::<&str>::into(self.match_type())]),
            Series::new("Game", &[self.game_number()]),
            Series::new("Tiebreak", &[self.tiebreak_number()]),
            Series::new("Stage", &[Into::<&str>::into(self.stage())]),
        ];

        DataFrame::new(v_s).unwrap()
        // let df = DataFrame::(
        //     "File" => &[self.path.file_stem().unwrap().to_str()],
        //     "Datetime" => &[date.unix_timestamp()],
        //     "Duration" => &[self.duration.as_millis() as u64],
        //     // "MatchID" => &[self.metadata.match_id.clone()],
        //     "MatchType" => &[self.metadata.match_type.map(Into::<&str>::into)],
        //     "Game" => &[self.metadata.game_number],
        //     "Tiebreak" => &[self.metadata.tiebreak_number],
        //     "Stage" => &[Into::<&str>::into(self.metadata.stage)],

        // ).unwrap();
    }
}

impl From<&InputStats> for DataFrame {
    fn from(value: &InputStats) -> Self {
        use crate::columns::InputStats as col;
        df!(
            col::Digital.into() => value.digital.clone(),
            col::Joystick.into() => value.joystick.clone(),
            col::Cstick.into() => value.cstick.clone(),
            col::AnalogTrigger.into() => value.analog_trigger.clone(),
            col::APM.into() => value.apm.clone(),
            col::TriggerPref.into() => value.trigger_pref.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            col::JumpPref.into() => value.jump_pref.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    ).unwrap()
    }
}

impl From<&WavedashStats> for DataFrame {
    fn from(val: &WavedashStats) -> Self {
        use crate::columns::WavedashStats as col;

        let vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index.clone()),
            Series::new(col::Waveland.into(), val.waveland.clone()),
            Series::new(col::Angle.into(), val.angle.clone()),
            Series::new(
                col::Direction.into(),
                val.direction
                    .iter()
                    .map(Into::<&str>::into)
                    .collect::<Vec<_>>(),
            ),
            StructChunked::new(
                col::StartPosition.into(),
                &[
                    Series::new(
                        "x",
                        val.start_position.iter().map(|p| p.x).collect::<Vec<_>>(),
                    ),
                    Series::new(
                        "y",
                        val.start_position.iter().map(|p| p.y).collect::<Vec<_>>(),
                    ),
                ],
            )
            .unwrap()
            .into_series(),
        ];

        DataFrame::new(vec_series).unwrap()
    }
}

impl WavedashStats {
    pub fn summary_df(&self) -> Option<DataFrame> {
        use crate::columns::WavedashStats as clm;
        let lf = Into::<DataFrame>::into(self).lazy();

        lf.select(&[
            col(clm::Waveland.into())
                .filter(col(clm::Waveland.into()).eq(lit(false)))
                .count()
                .alias("Wavedashes"),
            col(clm::Waveland.into())
                .filter(col(clm::Waveland.into()).eq(lit(true)))
                .count()
                .alias("Wavelands"),
            col(clm::Angle.into()).mean().alias("AvgAngle"),
        ])
        .collect()
        .ok()
    }
}

impl LCancelStats {
    pub fn lc_summary(&self) -> Option<DataFrame> {
        use crate::columns::LCancelStats as clm;
        let df = Into::<DataFrame>::into(self);
        let lf = df.clone().lazy();

        lf
            .select(&[col(clm::LCancel.into()).mean().alias("LCancelPercent")])
            .collect()
            .ok()
    }
}

impl From<&LCancelStats> for DataFrame {
    fn from(value: &LCancelStats) -> Self {
        use crate::columns::LCancelStats as col;
        let v_s = vec![
            Series::new(col::FrameIndex.into(), value.frame_index.clone()),
            Series::new(col::Attack.into(), value.attack.iter().map(|x| *x as u16).collect::<Vec<_>>()),
            Series::new(col::Stocks.into(), value.stocks.clone()),
            Series::new(col::Percent.into(), value.percent.clone()),
            Series::new(col::LCancel.into(), value.l_cancel.clone()),
            Series::new(col::TriggerFrame.into(), value.trigger_input_frame.clone()),
            Series::new(col::Position.into(), value.position.iter().map(Into::<&'static str>::into).collect::<Vec<_>>()),
            Series::new(col::Fastfall.into(), value.fastfall.clone()),
            Series::new(col::InputDuringHitlag.into(), value.during_hitlag.clone()),
        ];


        DataFrame::new(v_s).unwrap()
    }
}

impl DefenseStats {
    pub fn summary_df(&self) -> Option<DataFrame> {
        use crate::columns::DefenseStats as clm;
        let df = Into::<DataFrame>::into(self);
        let lf = df.clone().lazy();
        lf
            .select(&[
                col(clm::DamageTaken.into()).sum(),
                col(clm::DamageTaken.into()).count().alias("HitsTaken"),
                col(clm::DIEfficacy.into()).mean(),
                col(clm::LastHitBy.into())
                    .mode()
                    .implode()
                    .cast(DataType::List(Box::new(DataType::String)))
                    .alias("MostHitBy"),
                col(clm::StateBeforeHit.into())
                    .mode()
                    .implode()
                    .cast(DataType::List(Box::new(DataType::String)))
                    .alias("StateMostPunished"),
                col(clm::SDIInputs.into())
                    .list()
                    .len()
                    .sum()
                    .cast(DataType::Float32)
                    .div(lit(df.height() as u64))
                    .alias("SDIPerHit"),
                col(clm::KillsWithDI.into())
                    .filter(
                        col(clm::KillsSomeDI.into()).eq(lit(true)).and(
                            col(clm::KillsAllDI.into())
                                .eq(lit(false))
                                .and(col(clm::KillsWithDI.into()).eq(lit(false))),
                        ),
                    )
                    .count()
                    .alias("LivableHitsLived"),
                col(clm::KillsWithDI.into())
                    .filter(
                        col(clm::KillsSomeDI.into())
                            .eq(lit(true))
                            .and(col(clm::KillsAllDI.into()).eq(lit(false))),
                    )
                    .count()
                    .alias("LivableHits"),
                col(clm::HitlagFrames.into()).sum().alias("FramesInHitlag"),
                // col(clm::KillsWithDI.into())
                //     .filter(
                //         col(clm::KillsWithDI.into())
                //             .eq(lit(false))
                //             .and(col(clm::KillsSomeDI.into()).eq(lit(true)))
                //     )
                //     .count()
                //     .alias("DILives")
                //     ,
                // col(clm::KillsSomeDI.into()).filter(col(clm::KillsSomeDI.into()).eq(lit(true))).count().alias("CouldDie")
            ])
            .collect()
            .ok()
    }
}

impl From<&DefenseStats> for DataFrame {
    fn from(val: &DefenseStats) -> Self {
        use crate::columns::DefenseStats as col;

        let vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index.clone()),
            Series::new(col::Stocks.into(), val.stocks_remaining.clone()),
            Series::new(col::Percent.into(), val.percent.clone()),
            Series::new(col::DamageTaken.into(), val.damage_taken.clone()),
            Series::new(col::LastHitBy.into(), as_vec_static_str(val.last_hit_by.clone())),
            Series::new(
                col::StateBeforeHit.into(),
                as_vec_static_str(val.state_before_hit.clone()),
            ),
            Series::new(col::Grounded.into(), val.grounded.clone()),
            Series::new(col::CrouchCancel.into(), val.crouch_cancel.clone()),
            Series::new(col::VCancel.into(), val.v_cancel.clone()),
            Series::new(col::ASDI.into(), as_vec_arrow(val.asdi.clone())),
            Series::new(col::HitlagFrames.into(), val.hitlag_frames.clone()),
            Series::new(
                col::StickDuringHitlag.into(),
                val.stick_during_hitlag
                    .iter()
                    .map(|x| Series::new("", as_vec_arrow(x.clone())))
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                col::SDIInputs.into(),
                val.sdi_inputs
                    .iter()
                    .map(|x| Series::new("", as_vec_arrow(x.clone())))
                    .collect::<Vec<_>>(),
            ),
            StructChunked::new(
                col::HitlagStart.into(),
                &[
                    Series::new(
                        "x",
                        val.hitlag_start.iter().map(|p| p.x).collect::<Vec<_>>(),
                    ),
                    Series::new(
                        "y",
                        val.hitlag_start.iter().map(|p| p.y).collect::<Vec<_>>(),
                    ),
                ],
            )
            .unwrap()
            .into_series(),
            StructChunked::new(
                col::HitlagEnd.into(),
                &[
                    Series::new("x", val.hitlag_end.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.hitlag_end.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            StructChunked::new(
                col::DIStick.into(),
                &[
                    Series::new("x", val.di_stick.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.di_stick.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            StructChunked::new(
                col::Knockback.into(),
                &[
                    Series::new("x", val.kb.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.kb.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            StructChunked::new(
                col::DIKnockback.into(),
                &[
                    Series::new("x", val.di_kb.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.di_kb.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(col::KBAngle.into(), val.kb_angle.clone()),
            Series::new(col::DIKBAngle.into(), val.di_kb_angle.clone()),
            Series::new(col::DIEfficacy.into(), val.di_efficacy.clone()),
            Series::new(col::KillsWithDI.into(), val.kills_with_di.clone()),
            Series::new(col::KillsNoDI.into(), val.kills_no_di.clone()),
            Series::new(col::KillsAllDI.into(), val.kills_any_di.clone()),
            Series::new(col::KillsSomeDI.into(), val.kills_some_di.clone()),
        ];

        DataFrame::new(vec_series).unwrap()
    }
}

impl TechStats {
    pub fn summary_df(&self) -> Option<DataFrame> {
        use crate::columns::TechStats as clm;
        let df = Into::<DataFrame>::into(self);
        let lf = df.clone().lazy();

        lf
            .select(&[
                col(clm::TowardsCenter.into()).mean(),
                col(clm::TowardsOpnt.into()).mean(),
                col(clm::MissedTech.into()).mean(),
                col(clm::Punished.into()).mean(),
                col(clm::TechType.into())
                    .filter(
                        col(clm::TechType.into())
                            .eq(lit("MISSED_TECH_ROLL_LEFT"))
                            .or(col(clm::TechType.into()).eq(lit("ROLL_LEFT"))),
                    )
                    .count()
                    .alias("RollsLeft"),
                col(clm::TechType.into())
                    .filter(
                        col(clm::TechType.into())
                            .eq(lit("MISSED_TECH_ROLL_RIGHT"))
                            .or(col(clm::TechType.into()).eq(lit("ROLL_RIGHT"))),
                    )
                    .count()
                    .alias("RollsRight"),
                col(clm::TechType.into())
                    .filter(
                        col(clm::TechType.into())
                            .eq(lit("MISSED_TECH_GET_UP"))
                            .or(col(clm::TechType.into()).eq(lit("TECH_IN_PLACE"))),
                    )
                    .count()
                    .alias("InPlace"),
            ])
            .collect()
            .ok()
    }
}

impl From<&TechStats> for DataFrame {
    fn from(value: &TechStats) -> Self {
        use crate::columns::TechStats as clm;
        let v_s = vec![
            Series::new(clm::FrameIndex.into(), value.frame_index.clone()),
            Series::new(clm::Stocks.into(), value.stocks_remaining.clone()),
            Series::new(clm::Percent.into(), value.percent.clone()),
            Series::new(clm::LastHitBy.into(), as_vec_static_str(value.last_hit_by.clone())),
            Series::new(clm::InputFrame.into(), value.input_frame.clone()),
            Series::new(clm::DuringHitlag.into(), value.during_hitlag.clone()),
            Series::new(clm::MissedTech.into(), value.missed_tech.clone()),
            Series::new(clm::Lockout.into(), value.lockout.clone()),
            Series::new(clm::TechType.into(), as_vec_static_str(value.tech_type.clone())),
            Series::new(clm::JabReset.into(), value.jab_reset.clone()),
            Series::new(clm::Punished.into(), value.punished.clone()),
            StructChunked::new(
                clm::Position.into(),
                &[
                    Series::new("x", value.position.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", value.position.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(clm::Location.into(), as_vec_static_str(value.location.clone())),
            Series::new(clm::TowardsCenter.into(), value.towards_center.clone()),
            Series::new(clm::TowardsOpnt.into(), value.towards_opponent.clone()),
            Series::new(clm::OpntDistance.into(), value.opnt_distance.clone()),
        ];

        DataFrame::new(v_s).unwrap()
    }
}

impl From<PostFrames> for DataFrame {
    fn from(val: PostFrames) -> Self {
        let len = val.len();

        use crate::columns::PostFrame as col;
        let mut vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index),
            Series::new(col::Character.into(), val.character),
            Series::new(col::ActionState.into(), val.action_state),
            StructChunked::new(
                col::Position.into(),
                &[
                    Series::new("x", val.position.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.position.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(col::Orientation.into(), val.orientation),
            Series::new(col::Percent.into(), val.percent),
            Series::new(col::ShieldHealth.into(), val.shield_health),
            Series::new(col::LastAttackLanded.into(), val.last_attack_landed),
            Series::new(col::ComboCount.into(), val.combo_count),
            Series::new(col::LastHitBy.into(), val.last_hit_by),
            Series::new(col::Stocks.into(), val.stocks),
        ];

        if val.metadata.version.at_least(2, 0, 0) {
            vec_series.push(Series::new(
                col::StateFrame.into(),
                val.state_frame.unwrap(),
            ));
            vec_series.push(Series::new(col::Flags.into(), val.flags.unwrap()));
            vec_series.push(Series::new(col::MiscAS.into(), val.misc_as.unwrap()));
            vec_series.push(Series::new(
                col::IsGrounded.into(),
                val.is_grounded.unwrap(),
            ));
            vec_series.push(Series::new(
                col::LastGroundID.into(),
                val.last_ground_id.unwrap(),
            ));
            vec_series.push(Series::new(
                col::JumpsRemaining.into(),
                val.jumps_remaining.unwrap(),
            ));
            vec_series.push(Series::new(col::LCancel.into(), val.l_cancel.unwrap()));
        } else {
            vec_series.push(Series::new_null(col::StateFrame.into(), len));
            vec_series.push(Series::new_null(col::Flags.into(), len));
            vec_series.push(Series::new_null(col::MiscAS.into(), len));
            vec_series.push(Series::new_null(col::IsGrounded.into(), len));
            vec_series.push(Series::new_null(col::LastGroundID.into(), len));
            vec_series.push(Series::new_null(col::JumpsRemaining.into(), len));
            vec_series.push(Series::new_null(col::LCancel.into(), len));
        }

        if val.metadata.version.at_least(2, 1, 0) {
            vec_series.push(Series::new(
                col::HurtboxState.into(),
                val.hurtbox_state.unwrap(),
            ));
        } else {
            vec_series.push(Series::new_null(col::HurtboxState.into(), len));
        }

        if val.metadata.version.at_least(3, 5, 0) {
            vec_series.push(
                StructChunked::new(
                    col::AirVel.into(),
                    &[
                        Series::new(
                            "x",
                            val.air_velocity
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.x)
                                .collect::<Vec<_>>(),
                        ),
                        Series::new(
                            "y",
                            val.air_velocity
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.y)
                                .collect::<Vec<_>>(),
                        ),
                    ],
                )
                .unwrap()
                .into_series(),
            );
            vec_series.push(
                StructChunked::new(
                    col::Knockback.into(),
                    &[
                        Series::new(
                            "x",
                            val.knockback
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.x)
                                .collect::<Vec<_>>(),
                        ),
                        Series::new(
                            "y",
                            val.knockback
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.y)
                                .collect::<Vec<_>>(),
                        ),
                    ],
                )
                .unwrap()
                .into_series(),
            );
            vec_series.push(
                StructChunked::new(
                    col::GroundVel.into(),
                    &[
                        Series::new(
                            "x",
                            val.ground_velocity
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.x)
                                .collect::<Vec<_>>(),
                        ),
                        Series::new(
                            "y",
                            val.ground_velocity
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.y)
                                .collect::<Vec<_>>(),
                        ),
                    ],
                )
                .unwrap()
                .into_series(),
            );
        } else {
            vec_series.push(Series::new_null(col::AirVel.into(), len));
            vec_series.push(Series::new_null(col::Knockback.into(), len));
            vec_series.push(Series::new_null(col::GroundVel.into(), len));
        }

        if val.metadata.version.at_least(3, 8, 0) {
            vec_series.push(Series::new(
                col::HitlagRemaining.into(),
                val.hitlag_remaining.unwrap(),
            ));
        } else {
            vec_series.push(Series::new_null(col::HitlagRemaining.into(), len));
        }

        if val.metadata.version.at_least(3, 11, 0) {
            vec_series.push(Series::new(
                col::AnimationIndex.into(),
                val.animation_index.unwrap(),
            ));
        } else {
            vec_series.push(Series::new_null(col::AnimationIndex.into(), len));
        }

        if val.metadata.version.at_least(3, 16, 0) {
            vec_series.push(Series::new(
                col::InstanceHitBy.into(),
                val.instance_hit_by.unwrap(),
            ));
            vec_series.push(Series::new(
                col::InstanceID.into(),
                val.instance_id.unwrap(),
            ));
        } else {
            vec_series.push(Series::new_null(col::InstanceHitBy.into(), len));
            vec_series.push(Series::new_null(col::InstanceID.into(), len));
        }

        DataFrame::new(vec_series).unwrap()
    }
}

impl From<PreFrames> for DataFrame {
    fn from(val: PreFrames) -> Self {
        let len = val.len();

        use crate::columns::PreFrame as col;
        let mut vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index),
            Series::new(col::RandomSeed.into(), val.random_seed),
            Series::new(col::ActionState.into(), val.action_state),
            // wow polars is ugly in rust
            StructChunked::new(
                col::Position.into(),
                &[
                    Series::new("x", val.position.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.position.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(col::Orientation.into(), val.orientation),
            StructChunked::new(
                col::JoystickPos.into(),
                &[
                    Series::new("x", val.joystick.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.joystick.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            StructChunked::new(
                col::CstickPos.into(),
                &[
                    Series::new("x", val.cstick.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.cstick.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(col::EngineTrigger.into(), val.engine_trigger),
            Series::new(col::EngineButtons.into(), val.engine_buttons),
            Series::new(col::ControllerButtons.into(), val.controller_buttons),
            Series::new(col::ControllerL.into(), val.controller_l),
            Series::new(col::ControllerR.into(), val.controller_r),
        ];
        if val.metadata.version.at_least(1, 2, 0) {
            vec_series.push(Series::new(col::RawStickX.into(), val.raw_stick_x.unwrap()));
        } else {
            vec_series.push(Series::new_null(col::RawStickX.into(), len));
        }
        if val.metadata.version.at_least(1, 4, 0) {
            vec_series.push(Series::new(col::Percent.into(), val.percent.unwrap()));
        } else {
            vec_series.push(Series::new_null(col::Percent.into(), len));
        }
        if val.metadata.version.at_least(3, 15, 0) {
            vec_series.push(Series::new(col::RawStickY.into(), val.raw_stick_y.unwrap()));
        } else {
            vec_series.push(Series::new_null(col::RawStickY.into(), len));
        }

        DataFrame::new(vec_series).unwrap()
    }
}

#[allow(clippy::from_over_into)]
impl From<&ItemFrames> for DataFrame {
    fn from(val: &ItemFrames) -> DataFrame {
        let len = val.len();

        use crate::columns::ItemFrame as col;
        let mut vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index.clone()),
            Series::new(col::ItemID.into(), val.item_id.clone()),
            Series::new(col::State.into(), val.state.clone()),
            Series::new(col::Orientation.into(), val.orientation.clone()),
            StructChunked::new(
                col::Velocity.into(),
                &[
                    Series::new("x", val.velocity.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.velocity.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            StructChunked::new(
                col::Position.into(),
                &[
                    Series::new("x", val.position.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.position.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(col::DamageTaken.into(), val.damage_taken.clone()),
            Series::new(col::ExpirationTimer.into(), val.expiration_timer.clone()),
            Series::new(col::SpawnID.into(), val.spawn_id.clone()),
        ];

        if val.metadata.version.at_least(3, 2, 0) {
            vec_series.push(Series::new(
                col::MissileType.into(),
                val.missile_type.as_ref().unwrap().clone(),
            ));
            vec_series.push(Series::new(
                col::TurnipType.into(),
                val.turnip_type.as_ref().unwrap().clone(),
            ));
            vec_series.push(Series::new(col::Launched.into(), val.launched.as_ref().unwrap().clone()));
            vec_series.push(Series::new(
                col::ChargePower.into(),
                val.charge_power.as_ref().unwrap().clone(),
            ));
        } else {
            vec_series.push(Series::new_null(col::MissileType.into(), len));
            vec_series.push(Series::new_null(col::TurnipType.into(), len));
            vec_series.push(Series::new_null(col::Launched.into(), len));
            vec_series.push(Series::new_null(col::ChargePower.into(), len));
        }

        if val.metadata.version.at_least(3, 6, 0) {
            vec_series.push(Series::new(col::Owner.into(), val.owner.as_ref().unwrap().clone()));
        } else {
            vec_series.push(Series::new_null(col::Owner.into(), len));
        }

        if val.metadata.version.at_least(3, 16, 0) {
            vec_series.push(Series::new(
                col::InstanceID.into(),
                val.instance_id.as_ref().unwrap().clone(),
            ));
        } else {
            vec_series.push(Series::new_null(col::InstanceID.into(), len));
        }

        DataFrame::new(vec_series).unwrap()
    }
}

impl From<&ItemStats> for DataFrame {
    fn from(value: &ItemStats) -> Self {
        df!(
            "Item" => value.items.iter().map(Into::<&'static str>::into).collect::<Vec<_>>(),
            "Count" => value.counts.clone(),
        ).unwrap()
    }
}