# Rasta

Guitar/bass effect unit.

## Example audio

[[mp3]](http://rickyhan.com/out.mp3)

## Getting started

1. Run a JACK server and set [appropriate sample rate](https://askubuntu.com/questions/539406/how-to-avoid-xrun-callback-skips) to achieve low enough latency.
2. On Ubuntu install `libjack-dev` or equivalent for other system
3. Load supplied patchbay definition
5. `cargo run --release`
6. Type: `c in out` to chain from input to output

## How to use

| Command                  | Explanation                                       |
|--------------------------|---------------------------------------------------|
| c in delay out           | connect input to delay pedal then to out          |
| c in aw out              | autowah                                           |
| a delay2 delay           | add a delay effect named delay2                   |
| s delay2 delay 0.14      | Set pedal "delay" parameter delay to 0.14 seconds |
| s delay2 feedback 0.8    | Set feedback to 0.8                               |
| c in aw delay delay2 out | daisy chain together                              |
| p                        | print current graph definition                    |
| b aw                     | bypass autowah pedal                              |
| b                        | bypass all effects                                |

## License

MIT