html! {
  <div class="space-y-6">
      <Ul spacing="space-y-1">
          <li>{ "Default disc marker" }</li>
          <li>{ "Another list item" }</li>
      </Ul>

      <Ul marker_type={MarkerType::Decimal} spacing="space-y-1">
          <li>{ "Decimal style marker" }</li>
          <li>{ "Item two" }</li>
      </Ul>

      <Ul marker_type={MarkerType::None}>
          <li class="bg-gray-100 px-2 py-1 rounded">{ "No marker, styled manually" }</li>
          <li class="bg-gray-100 px-2 py-1 rounded">{ "Second manual item" }</li>
      </Ul>
  </div>
}
