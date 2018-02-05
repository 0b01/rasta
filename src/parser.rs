use effects::CtrlMsg;

pub fn parse_input(cmd: &str) -> CtrlMsg {
    use self::CtrlMsg::*;

    if cmd == "t" {
        Tuner
    } else

    if cmd == "b" {
        Bypass
    } else

    if cmd.starts_with("b") {
        let tokens = cmd[2..]
            .split(" ")
            .collect::<Vec<&str>>();
        let mut chain = vec![];
        for token in tokens.into_iter() {
            chain.push(BypassPedal(token.to_owned()));
        }
        Chain(chain)
    } else

    if cmd == "p" {
        Connections
    } else

    if cmd.starts_with("d") {
        let tokens = cmd[2..]
            .split(" ")
            .collect::<Vec<&str>>();
        let mut chain = vec![];
        for a in tokens.into_iter() {
            chain.push(Disconnect(a.to_owned()));
        }
        Chain(chain)
    } else

    if cmd.starts_with("s") {
        let tokens = cmd[2..]
            .split(" ")
            .collect::<Vec<&str>>();
        let pedal_name = tokens[0].to_owned();
        let conf_name = tokens[1].to_owned();
        let val = tokens[2].parse::<f32>().unwrap();
        Set(pedal_name, conf_name, val)

    } else
    
    if cmd.starts_with("c") {
        // allow daisy chaining:
        // c in delay overdrive out
        let tokens = cmd[2..]
            .split(" ")
            .collect::<Vec<&str>>();

        let mut chain = vec![];
        for (a, b) in tokens.iter().zip(tokens[1..].into_iter()) {
            let inp = a.to_owned().to_owned();
            let outp = b.to_owned().to_owned();

            chain.push(Connect(inp, outp))
        }

        Chain(chain)
    } else
    
    if cmd.starts_with("a") {
        let tokens = cmd.split(" ").collect::<Vec<&str>>();
        let name = tokens[1].to_owned();
        let eff_type = tokens[2].to_owned();

        Add(name, eff_type)

    } else {
        Bypass
    }
}