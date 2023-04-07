use crate::entities::parser::NotEmptyStr;
use crate::entities::{
  AlterFilterParams, AlterFilterPayloadPB, AlterSortParams, AlterSortPayloadPB,
  CalendarLayoutSettingsPB, DeleteFilterParams, DeleteFilterPayloadPB, DeleteGroupParams,
  DeleteGroupPayloadPB, DeleteSortParams, DeleteSortPayloadPB, InsertGroupParams,
  InsertGroupPayloadPB, RepeatedFilterPB, RepeatedGroupSettingPB, RepeatedSortPB,
};
use crate::services::setting::CalendarLayoutSetting;
use collab_database::views::DatabaseLayout;
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use flowy_error::ErrorCode;
use std::convert::TryInto;
use strum_macros::EnumIter;

/// [DatabaseViewSettingPB] defines the setting options for the grid. Such as the filter, group, and sort.
#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct DatabaseViewSettingPB {
  #[pb(index = 1)]
  pub current_layout: DatabaseLayoutPB,

  #[pb(index = 2)]
  pub layout_setting: LayoutSettingPB,

  #[pb(index = 3)]
  pub filters: RepeatedFilterPB,

  #[pb(index = 4)]
  pub group_settings: RepeatedGroupSettingPB,

  #[pb(index = 5)]
  pub sorts: RepeatedSortPB,
}

#[derive(Debug, Clone, PartialEq, Eq, ProtoBuf_Enum, EnumIter)]
#[repr(u8)]
pub enum DatabaseLayoutPB {
  Grid = 0,
  Board = 1,
  Calendar = 2,
}

impl std::default::Default for DatabaseLayoutPB {
  fn default() -> Self {
    DatabaseLayoutPB::Grid
  }
}

impl std::convert::From<DatabaseLayout> for DatabaseLayoutPB {
  fn from(rev: DatabaseLayout) -> Self {
    match rev {
      DatabaseLayout::Grid => DatabaseLayoutPB::Grid,
      DatabaseLayout::Board => DatabaseLayoutPB::Board,
      DatabaseLayout::Calendar => DatabaseLayoutPB::Calendar,
    }
  }
}

impl std::convert::From<DatabaseLayoutPB> for DatabaseLayout {
  fn from(layout: DatabaseLayoutPB) -> Self {
    match layout {
      DatabaseLayoutPB::Grid => DatabaseLayout::Grid,
      DatabaseLayoutPB::Board => DatabaseLayout::Board,
      DatabaseLayoutPB::Calendar => DatabaseLayout::Calendar,
    }
  }
}

#[derive(Default, ProtoBuf)]
pub struct DatabaseSettingChangesetPB {
  #[pb(index = 1)]
  pub view_id: String,

  #[pb(index = 2)]
  pub layout_type: DatabaseLayoutPB,

  #[pb(index = 3, one_of)]
  pub alter_filter: Option<AlterFilterPayloadPB>,

  #[pb(index = 4, one_of)]
  pub delete_filter: Option<DeleteFilterPayloadPB>,

  #[pb(index = 5, one_of)]
  pub insert_group: Option<InsertGroupPayloadPB>,

  #[pb(index = 6, one_of)]
  pub delete_group: Option<DeleteGroupPayloadPB>,

  #[pb(index = 7, one_of)]
  pub alter_sort: Option<AlterSortPayloadPB>,

  #[pb(index = 8, one_of)]
  pub delete_sort: Option<DeleteSortPayloadPB>,
}

impl TryInto<DatabaseSettingChangesetParams> for DatabaseSettingChangesetPB {
  type Error = ErrorCode;

  fn try_into(self) -> Result<DatabaseSettingChangesetParams, Self::Error> {
    let view_id = NotEmptyStr::parse(self.view_id)
      .map_err(|_| ErrorCode::ViewIdIsInvalid)?
      .0;

    let insert_filter = match self.alter_filter {
      None => None,
      Some(payload) => Some(payload.try_into()?),
    };

    let delete_filter = match self.delete_filter {
      None => None,
      Some(payload) => Some(payload.try_into()?),
    };

    let insert_group = match self.insert_group {
      Some(payload) => Some(payload.try_into()?),
      None => None,
    };

    let delete_group = match self.delete_group {
      Some(payload) => Some(payload.try_into()?),
      None => None,
    };

    let alert_sort = match self.alter_sort {
      None => None,
      Some(payload) => Some(payload.try_into()?),
    };

    let delete_sort = match self.delete_sort {
      None => None,
      Some(payload) => Some(payload.try_into()?),
    };

    Ok(DatabaseSettingChangesetParams {
      view_id,
      layout_type: self.layout_type.into(),
      insert_filter,
      delete_filter,
      insert_group,
      delete_group,
      alert_sort,
      delete_sort,
    })
  }
}

pub struct DatabaseSettingChangesetParams {
  pub view_id: String,
  pub layout_type: DatabaseLayout,
  pub insert_filter: Option<AlterFilterParams>,
  pub delete_filter: Option<DeleteFilterParams>,
  pub insert_group: Option<InsertGroupParams>,
  pub delete_group: Option<DeleteGroupParams>,
  pub alert_sort: Option<AlterSortParams>,
  pub delete_sort: Option<DeleteSortParams>,
}

impl DatabaseSettingChangesetParams {
  pub fn is_filter_changed(&self) -> bool {
    self.insert_filter.is_some() || self.delete_filter.is_some()
  }
}

#[derive(Debug, Eq, PartialEq, Default, ProtoBuf, Clone)]
pub struct LayoutSettingPB {
  #[pb(index = 1)]
  pub calendar: CalendarLayoutSettingsPB,
}

impl LayoutSettingPB {
  pub fn new() -> Self {
    Self::default()
  }
}

#[derive(Debug, Default, Clone)]
pub struct LayoutSettingParams {
  pub calendar: Option<CalendarLayoutSetting>,
}

#[derive(Debug, Eq, PartialEq, Default, ProtoBuf, Clone)]
pub struct UpdateLayoutSettingPB {
  #[pb(index = 1)]
  pub view_id: String,

  #[pb(index = 2)]
  pub layout_setting: LayoutSettingPB,
}

// #[derive(Debug)]
// pub struct UpdateLayoutSettingParams {
//   pub view_id: String,
//   pub layout_setting: LayoutSettingParams,
// }
//
// impl TryInto<UpdateLayoutSettingParams> for UpdateLayoutSettingPB {
//   type Error = ErrorCode;
//
//   fn try_into(self) -> Result<UpdateLayoutSettingParams, Self::Error> {
//     let view_id = NotEmptyStr::parse(self.view_id)
//       .map_err(|_| ErrorCode::ViewIdIsInvalid)?
//       .0;
//
//     let layout_setting: LayoutSettingParams = self.layout_setting.into();
//
//     Ok(UpdateLayoutSettingParams {
//       view_id,
//       layout_setting,
//     })
//   }
// }
