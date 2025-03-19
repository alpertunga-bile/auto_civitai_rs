# auto_civitai_rs

[![Docs.rs](https://docs.rs/auto_civitai_rs/badge.svg)](https://docs.rs/auto_civitai_rs)
[![CI](https://github.com/alpertunga-bile/auto_civitai_rs/workflows/CI/badge.svg)](https://github.com/alpertunga-bile/auto_civitai_rs/actions)

## Usage

1. Clone the repository
2. Run 
```bash
cargo run --release
```

### Config File

- To use, ```config.json``` file has to be present
- Below fields have to be included:

|       Key        | Value (and Options)                             | Definition                                |
| :--------------: | :---------------------------------------------- | :---------------------------------------- |
|      output      | string                                          | Output relative/real path of the dataset  |
|      limit       | number, [1, 200]                                | Total images in one page                  |
|       nsfw       | string, [true, false None, Soft, Mature, X]     | NSFW level of the images                  |
|       sort       | string, [Most Reactions, Most Comments, Newest] | Sort with given parameter                 |
|      period      | string, [AllTime, Year, Month, Week, Day]       | Time period of the query                  |
|    start_page    | number, [0, 49999]                              | Starting cursor value                     |
|   total_pages    | number, [1, 250]                                | Total pages wanted to be fetched          |
|  wanted_prompts  | string array                                    | Wanted keywords to be in the prompt       |
| unwanted_prompts | string array                                    | Unwanted keywords to not be in the prompt |

- Example config file:

```json
{
  "output": "dataset",
  "limit": 200,
  "nsfw": "None",
  "sort": "Most Reactions",
  "period": "AllTime",
  "start_page": 0,
  "total_pages": 250,
  "wanted_prompts": ["dog"],
  "unwanted_prompts": ["bad"]
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
