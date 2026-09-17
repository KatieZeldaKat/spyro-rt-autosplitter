> [!warning]
> 
> This is an archived version of the wiki that used to be enabled on https://github.com/KatieZeldaKat/spyro-rt-autosplitter. It contains outdated information, so do not use it as a source of truth!

---

LiveSplit can only be installed on Windows. Although there are workarounds to run this on other operating systems like Linux, it is recommended to use something like LiveSplit One Druid instead.

# Instructions

First, install [LiveSplit](https://livesplit.org/downloads/). Once it's running, load a split file for Spyro: Reignited (there are two clear split `.zip` files in the [Spyro: Reignited Trilogy (PC) page](https://www.speedrun.com/spyrortpc/resources) if you need some splits to work with). Go into `Edit Splits`. If an auto-splitter is currently activated, click `Deactivate` on it. Now you can set up the auto-splitter from this repository.

Create or edit an existing LiveSplit Layout (`.lsl`) file. To do this, right click LiveSplit and select `Edit Layout`. Add an `Auto Splitting Runtime` component using `+` -> `Control` -> `Auto Splitting Runtime`. To edit this, click `Layout Settings` -> `Auto Splitting Runtime` -> `Browse...`. Select the `spyro-rt-autosplitter.wasm` file. There should now be a list of settings for the splitter. This layout is where any auto-splitter settings are stored, so keep a different layout for each category you run that requires different settings. Modify any settings you desire and save the layout.
