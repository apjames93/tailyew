html! {
  <div class="flex flex-col items-start bg-gray-100 p-4 rounded w-full max-w-sm text-sm">
      <span>{ "Above the spacer" }</span>
      <Spacer size={24} />
      <span>{ "Below the 24px vertical spacer" }</span>
      <Spacer horizontal=true size={40} class="bg-gray-400 my-2" />
      <span>{ "This line is visually offset by a horizontal spacer above" }</span>


      <Spacer /> // vertical 16px

      <Spacer size={32} /> // vertical 32px

      <Spacer horizontal={true} size={100} class="bg-red-500" />

      <Spacer class="hidden md:block" /> // responsive spacer
  </div>
}
