html! {
  <div class="space-y-4">
      <Typo tag={TagType::H1}>{ "This is an H1" }</Typo>
      <Typo tag={TagType::H2}>{ "This is an H2" }</Typo>
      <Typo tag={TagType::H3}>{ "This is an H3" }</Typo>
      <Typo tag={TagType::H4}>{ "This is an H4" }</Typo>
      <Typo tag={TagType::H5}>{ "This is an H5" }</Typo>
      <Typo tag={TagType::H6}>{ "This is an H6" }</Typo>
      <Typo tag={TagType::BlockQuote}>{ "This is an BlockQuote" }</Typo>
      <Typo tag={TagType::Emphasis}>{ "This is an Emphasis" }</Typo>
      <Typo tag={TagType::Strong}>{ "This is an Strong" }</Typo>
      <Typo tag={TagType::P}>{ "This is a paragraph of text using the P tag." }</Typo>
      <Typo tag={TagType::Span} class="bg-yellow-100 px-1 rounded">{ "This is a styled span" }</Typo>
  </div>
}
