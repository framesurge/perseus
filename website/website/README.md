# Website

This directory contains the website for Perseus, which is hosted at <https://arctic-hen7.github.io/perseus>!

## Why is this in `website/website`?

So that using the local version of Perseus rather than the most recently published release works without any further changes. In development, this is designed to work for the `examples/` directory, using `../../` to access `packages`. We mimic the same file structure here.

## Comparisons

The website includes a [comparisons page](https://arctic-hen7.github.io/perseus/comparisons), which compares Perseus to a number of other frameworks. Of course, there are _a lot_ of frameworks out there, so we highly encourage contributions to this! It's designed to be quite easy to contribute to, just add a new file called `website/website/comparisons/framework.json` (substituting `framework` for the name of the framework) and fill in the following framework details:

-   `name`: `String`,
-   `supports_ssg`: `"full"`/`"partial"`/`"none"`,
-   `supports_ssr`: `"full"`/`"partial"`/`"none"`,
-   `supports_ssr_ssg_same_page`: `"full"`/`"partial"`/`"none"`,
-   `supports_i18n`: `"full"`/`"partial"`/`"none"`,
-   `supports_incremental`: `"full"`/`"partial"`/`"none"`,
-   `supports_revalidation`: `"full"`/`"partial"`/`"none"`,
-   `inbuilt_cli`: `"full"`/`"partial"`/`"none"`,
-   `inbuilt_routing`: `"full"`/`"partial"`/`"none"`,
-   `supports_shell`: `"full"`/`"partial"`/`"none"`,
-   `supports_deployment`: `"full"`/`"partial"`/`"none"`,
-   `supports_exporting`: `"full"`/`"partial"`/`"none"`,
-   `language`: `String`,
-   `homepage_lighthouse_desktop`: `u8`,
-   `homepage_lighthouse_mobile`: `u8`

### Lighthouse Scores

For consistency, we generate all Lighthouse scores through [PageSpeed Insights](https://developers.google.com/speed/pagespeed/insights). As this metric can vary slightly between machines and runs, it's advised to run it more than once and take an average (rounding up). Maintainers will check these themselves, and if there's any major discrepancy (>5 points), you may be asked to provide a screenshot from your system. Maintainers reserve the right to determine the final verdict on which score to use in the event of a conflict.
