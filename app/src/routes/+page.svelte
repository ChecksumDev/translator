<script lang="ts">
  import EncodePage from "./encode.svelte";
  import DecodePage from "./decode.svelte";
  import SettingsPage from "./settings.svelte";

  let currentComponent = "encode";
  let isSwitching = false;

  let encodeActive = "active";
  let decodeActive = "notactive";
  let settingsActive = "notactive";

  let encodeSelected = "bg-stone-800";
  let decodeSelected = "bg-stone-800";
  let settingsSelected = "bg-stone-800";

  if (encodeActive == "active"){
    encodeSelected = "bg-stone-700";
  } else {
    encodeSelected = "bg-stone-800";
  }
  if (decodeActive == "active"){
    decodeSelected = "bg-stone-700";
  } else {
    decodeSelected = "bg-stone-800";
  }
  if (settingsActive == "active"){
    settingsSelected = "bg-stone-700";
  } else {
    settingsSelected = "bg-stone-800";
  }
  
  function switchToEncode() {
    if (!isSwitching) {
      isSwitching = true;
      setTimeout(() => {
        currentComponent = "encode";
        isSwitching = false;
        encodeActive = "active"; decodeActive = "notactive"; settingsActive = "notactive";
        encodeSelected = "bg-stone-700";
        decodeSelected = "bg-stone-800";
        settingsSelected = "bg-stone-800";
      }, 3);
    }
  }
  function switchToDecode() {
    if (!isSwitching) {
      isSwitching = true;
      setTimeout(() => {
        currentComponent = "decode";
        isSwitching = false;
        encodeActive = "notactive"; decodeActive = "active"; settingsActive = "notactive";
        encodeSelected = "bg-stone-800";
        decodeSelected = "bg-stone-700";
        settingsSelected = "bg-stone-800";
      }, 3);
    }
  }
  function switchToSettings() {
    if (!isSwitching) {
      isSwitching = true;
      setTimeout(() => {
        currentComponent = "settings";
        isSwitching = false;
        encodeActive = "notactive"; decodeActive = "notactive"; settingsActive = "active";
        encodeSelected = "bg-stone-800";
        decodeSelected = "bg-stone-800";
        settingsSelected = "bg-stone-700";
      }, 3);
    }
  }
</script>

<main class="w-screen h-screen bg-stone-950">
  <top class="fixed top-0 w-screen h-16 bg-stone-900">
    <top-container class="grid grid-cols-3">
      <div class="w-40 h-16 flex items-center">
        LOGO HERE
      </div>
      <div class="relative left-10 b-auto w-50 h-16 flex flex-row items-center gap-5">
        <button class="w-20 h-8 {encodeSelected} rounded-lg flex items-center justify-center" on:click={switchToEncode}>
          <p class="font-sans text-stone-100 font-semibold">Encode</p>
        </button>
        <button class="w-20 h-8 {decodeSelected} rounded-lg flex items-center justify-center" on:click={switchToDecode}>
          <p class="font-sans text-stone-100 font-semibold">Decode</p>
        </button>
      </div>
      <button class="absolute right-0 w-6 h-16 {settingsSelected} rounded-l-lg" on:click={switchToSettings}>
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="rgb(245 245 244)"><path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm112-260q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Z"/></svg>
      </button>
    </top-container>
  </top>
  <content class="fixed top-16 w-screen h-screen bg-stone-950">
    {#if currentComponent === "encode"}
      <div>
        {#if !isSwitching}
          <EncodePage/>
        {/if}
      </div>
    {/if}
    {#if currentComponent === "decode"}
      <div>
        {#if !isSwitching}
          <DecodePage/>
        {/if}
      </div>
    {/if}
    {#if currentComponent === "settings"}
      <div>
        {#if !isSwitching}
          <SettingsPage/>
        {/if}
      </div>
    {/if}
  </content>
</main>