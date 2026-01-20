html! {
  <div class="space-y-4">
      <Typo tag={TagType::H1}>{ "This is an H1" }</Typo>
      <Typo tag={TagType::H2}>{ "This is an H2" }</Typo>
      <Typo tag={TagType::H3}>{ "This is an H3" }</Typo>
      <Typo tag={TagType::H4}>{ "This is an H4" }</Typo>
      <Typo tag={TagType::H5}>{ "This is an H5" }</Typo>
      <Typo tag={TagType::H6}>{ "This is an H6" }</Typo>
      <Typo tag={TagType::BlockQuote}>{ "This is a BlockQuote" }</Typo>
      <Typo tag={TagType::Emphasis}>{ "This is emphasized text" }</Typo>
      <Typo tag={TagType::Strong}>{ "This is strong text" }</Typo>

      <Typo tag={TagType::P}>
          { "This is a paragraph with a description. " }
          <Typo tag={TagType::Span} id="desc" class={classes!("text-sm", "text-gray-500")}>
              { "This description is referenced by the paragraph." }
          </Typo>
      </Typo>

      <Typo
          tag={TagType::P}
          aria_describedby="desc"
          style={Some(AttrValue::from("color: var(--brand, #2563eb);"))}
      >
          { "This paragraph references the description below." }
      </Typo>

      <Typo
          tag={TagType::Error}
          aria_label="Error: something went wrong"
      >
          { "Something went wrong" }
      </Typo>

      <Typo
          tag={TagType::Span}
          class={classes!("bg-yellow-100", "px-1", "rounded")}
          aria_label="Highlighted span"
      >
          { "This is a styled span" }
      </Typo>
  </div>
}
