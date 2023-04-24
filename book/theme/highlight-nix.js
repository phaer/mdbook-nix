/*
 * Hacky way to integrate a non-default language for highlight.js with
 * mdbook. See https://github.com/rust-lang/mdBook/issues/657
 *
 * The ugliest thing is that it re-initializes highlight.js, and vendors
 * the nix language definition
 * from https://github.com/highlightjs/highlight.js/blob/01e096544b09d2248de9d942efefa9ad92228f52/src/languages/nix.js
 *
 * FIXME: PRs with a better approach a more than welcome
 * */

document.addEventListener("DOMContentLoaded",function(){
  const KEYWORDS = {
    keyword: [
      "rec",
      "with",
      "let",
      "in",
      "inherit",
      "assert",
      "if",
      "else",
      "then"
    ],
    literal: [
      "true",
      "false",
      "or",
      "and",
      "null"
    ],
    built_in: [
      "import",
      "abort",
      "baseNameOf",
      "dirOf",
      "isNull",
      "builtins",
      "map",
      "removeAttrs",
      "throw",
      "toString",
      "derivation"
    ]
  };
  const ANTIQUOTE = {
    className: 'subst',
    begin: /\$\{/,
    end: /\}/,
    keywords: KEYWORDS
  };
  const ESCAPED_DOLLAR = {
    className: 'char.escape',
    begin: /''\$/,
  };
  const ATTRS = {
    begin: /[a-zA-Z0-9-_]+(\s*=)/,
    returnBegin: true,
    relevance: 0,
    contains: [
      {
        className: 'attr',
        begin: /\S+/,
        relevance: 0.2
      }
    ]
  };
  const STRING = {
    className: 'string',
    contains: [ ESCAPED_DOLLAR, ANTIQUOTE ],
    variants: [
      {
        begin: "''",
        end: "''"
      },
      {
        begin: '"',
        end: '"'
      }
    ]
  };
  const EXPRESSIONS = [
    hljs.NUMBER_MODE,
    hljs.HASH_COMMENT_MODE,
    hljs.C_BLOCK_COMMENT_MODE,
    STRING,
    ATTRS
  ];
  ANTIQUOTE.contains = EXPRESSIONS;
  hljs.registerLanguage("nix", (hljs) => ({
    name: 'Nix',
    aliases: [ "nixos" ],
    keywords: KEYWORDS,
    contains: EXPRESSIONS
  }));
  hljs.initHighlightingOnLoad();
})
