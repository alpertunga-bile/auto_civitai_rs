# auto_civitai_rs

[![CI](https://github.com/alpertunga-bile/auto_civitai_rs/workflows/CI/badge.svg)](https://github.com/alpertunga-bile/auto_civitai_rs/actions)

## Features

- Parallel url fetch and processing
- Filter images with wanted and unwanted prompt keywords
- Postprocessing image prompts to made them more stable.
- If the dataset is present in given path, the dataset is updated otherwise created new dataset.
- The dataset is made unique with using image urls as key.

## Usage

- You have two choices:
  1. Use the prebuild binaries from the releases page
  2. Build from source

### Build

1. Clone the repository
2. Run

```bash
cargo run --release
```

### Config File

- To use, `config.json` file has to be present
- Below fields have to be included:

|       Key        | Value (and Options)                             | Definition                                |
| :--------------: | :---------------------------------------------- | :---------------------------------------- |
|      output      | string                                          | Output relative/real path of the dataset  |
|      limit       | number, [1, 200]                                | Total images in one page                  |
|       nsfw       | string, [true, false, None, Soft, Mature, X]    | NSFW level of the images                  |
|       sort       | string, [Most Reactions, Most Comments, Newest] | Sort with given parameter                 |
|      period      | string, [AllTime, Year, Month, Week, Day]       | Time period of the query                  |
|    start_page    | number, [0, 50000)                              | Starting cursor value                     |
|   total_pages    | number, [1, 50000]                              | Total pages wanted to be fetched          |
|  wanted_prompts  | string array                                    | Wanted keywords to be in the prompt       |
| unwanted_prompts | string array                                    | Unwanted keywords to not be in the prompt |

- [x] This constraint has to be followed: **limit** \* **total_pages** <= 50000

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

## Dataset Schema

- [x] The columns are written in order.

|       Name        |  Type  | Default Value |
| :---------------: | :----: | :-----------: |
|      prompt       | string |  "undefined"  |
|  negative_prompt  | string |  "undefined"  |
|       seed        |  u64   |       0       |
|       steps       |  u64   |       0       |
|      sampler      | string |  "undefined"  |
|        cfg        |  f64   |      0.0      |
|     clip_skip     |  u64   |       0       |
|     resources     | string |  "undefined"  |
| civitai_resources | string |  "undefined"  |
|        url        | string |  "undefined"  |
|    base_model     | string |  "undefined"  |
|    nsfw_level     | string |  "undefined"  |
|       type        | string | "undefined "  |
|     cry_count     |  u64   |       0       |
|    like_count     |  u64   |       0       |
|   dislike_count   |  u64   |       0       |
|   comment_count   |  u64   |       0       |

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
