# Breaking changes

Since there is no stable release yet, there is also no proper changelog yet. But
since not everyone might want to dive through commit messages to find out what's
new and what's changed, this document lists all breaking changes in reverse
chronological order. If a new feature did not require any changes to existing
code then it will not be listed here.

## [2022-09-22]

- `nih_plug_egui` has been updated from egui 0.17 to egui 0.19.
- `nih_plug_vizia` has been updated. Custom widgets will need to be updated
  because of changes Vizia itself.

## [2022-09-06]

- Parameter values are now accessed using `param.value()` instead of
  `param.value`, with `param.value()` being an alias for the existing
  `param.plain_value()` function. The old approach, while perfectly safe in
  practice, was technically unsound because it used mutable pointers to
  parameters that may also be simultaneously read from in an editor GUI. With
  this change the parameters now use actual relaxed atomic stores and loads to
  avoid mutable aliasing, which means the value fields are now no longer
  directly accessible.

## [2022-09-04]

- `Smoother::next_block_mapped()` and `Smoother::next_block_exact_mapped()` have
  been redesigned. They now take an index of the element being generated and the
  float representation of the smoothed value. This makes it easier to use them
  for modulation, and it makes it possible to smoothly modulate integers and
  other stepped parameters. Additionally, the mapping functions are now also
  called for every produced value, even if the smoother has already finished
  smoothing and is always producing the same value.

## [2022-08-19]

- `Plugin::DEFAULT_NUM_INPUTS` and `Plugin::DEFAULT_NUM_OUTPUTS` have been
  renamed to `Plugin::DEFAULT_INPUT_CHANNELS` and
  `Plugin::DEFAULT_OUTPUT_CHANNELS` respectively to avoid confusion as these
  constants only affect the main input and output.
- Standalones now use the plugin's default input and output channel counts
  instead of always defaulting to two inputs and two outputs.

## [2022-07-18]

- `IntRange` and `FloatRange` no longer have min/max methods and instead have
  next/previous step methods. This is for better compatibility with the new
  reversed ranges.

## [2022-07-06]

- The block smoothing API has been reworked. Instead of `Smoother`s having their
  own built-in block buffer, you now need to provide your own mutable slice for
  the smoother to fill. This makes the API easier to understand, more flexible,
  and it allows cloning smoothers without worrying about allocations.In
  addition, the new implementation is much more efficient when the smoothing
  period has ended before or during the block.
- There are new `NoteEvent::PolyModulation` and `NoteEvent::MonoAutomation`
  events as part of polyphonic modulation support for CLAP plugins.

## [2022-07-05]

- The `ClapPlugin::CLAP_HARD_REALTIME` constant was moved to the general
  `Plugin` trait as `Plugin::HARD_REALTIME_ONLY`, and best-effort support for
  VST3 has been added.

## [2022-07-04]

- There is a new `NoteEvent::Choke` event the host can send to a plugin to let
  it know that it should immediately terminate all sound associated with a voice
  or a key.
- There is a new `NoteEvent::VoiceTerminated` event a plugin can send to let the
  host know a voice has been terminated. This needs to be output by CLAP plugins
  that support polyphonic modulation.
- Most `NoteEvent` variants now have an additional `voice_id` field.
- The `CLAP_DESCRIPTION`, `CLAP_MANUAL_URL`, and `CLAP_SUPPORT_URL` associated
  constants from the `ClapPlugin` are now optional and have the type
  `Option<&'static str>` instead of `&'static str`.

## [2022-07-02]

- The `Params::serialize_fields()` and `Params::deserialize_fields()` methods
  and the `State` struct now use `BTreeMap`s instead of `HashMap`s so the order
  is consistent the plugin's state to JSON multiple times. These things are part
  of NIH-plug's internals, so unless you're implementing the `Params` trait by
  hand you will not notice any changes.

## [2022-06-01]

- The `ClapPlugin::CLAP_FEATURES` field now uses an array of `ClapFeature`
  values instead of `&'static str`s. CLAP 0.26 contains many new predefined
  features, and the existing ones now use dashes instead of underscores. Custom
  features are still possible using `ClapFeature::Custom`.

## [2022-05-27]

- The `Plugin::initialize()` method now takes a `&mut impl InitContext` instead
  of a `&mut impl ProcessContext`.
- `Plugin::process()` now takes a new `aux: &mut AuxiliaryBuffers` parameter.
  This was needed to allow auxiliary (sidechain) inputs and outputs.

## [2022-05-22]

- Previously calling `param.non_automatable()` when constructing a parameter
  also made the parameter hidden. Hiding a parameter is now done through
  `param.hide()`, while `param.non_automatable()` simply makes it so that the
  parameter can only be changed manually and not through automation or
  modulation.
- The current processing mode is now stored in `BufferConfig`. Previously this
  could be fetched through a function on the `ProcessContext`, but this makes
  more sense as it remains constant until a plugin is deactivated. The
  `BufferConfig` now contains a field for the minimum buffer size that may or
  may not be set depending on the plugin API.

## ...

Who knows what happened at this point!
