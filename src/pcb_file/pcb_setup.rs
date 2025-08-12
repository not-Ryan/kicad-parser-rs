use super::PcbStackUpSettings;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PcbSetup {
  /// The optional STACK_UP_SETTINGS define the parameters required to manufacture the board.
  pub stack_up_settings: Option<PcbStackUpSettings>,
  /// The pad_to_mask_clearance token defines the clearance between footprint pads and the solder mask.
  pub pad_to_mask_clearance: f64,
  /// The optional solder_mask_min_width defines the minimum solder mask width. If not defined, the minimum width is zero.
  pub solder_mask_min_width: Option<f64>,
  /// The optional pad_to_paste_clearance defines the clearance between footprint pads and the solder paste layer. If not defined, the clearance is zero.
  pub pad_to_paste_clearance: Option<f64>,
  /// The optional pad_to_paste_clearance_ratio is the percentage (from 0 to 100) of the footprint pad to make the solder paste. If not defined, the ratio is 100% (the same size as the pad).
  pub pad_to_paste_clearance_ratio: Option<f64>,
  /// The optional aux_axis_origin defines the auxiliary origin if it is set to anything other than (0,0).
  pub aux_axis_origin: Option<(f64, f64)>,
  /// The optional grid_origin defines the grid original if it is set to anything other than (0,0).
  pub grid_origin: Option<(f64, f64)>,
}
