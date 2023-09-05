use serde_json::json;

fn main() {}

#[test]
fn test_serde_dserilize() {
    let parser = Some(json!({"parser":
    {
        "parse":
        {
            "1":
            {
                "as": "SMALLINT UNSIGNED",
                "alias": "1"
            },
            "7":
            {
                "as": "TIMESTAMP(us)",
                "alias": "7"
            },
            "111":
            {
                "as": "INT",
                "alias": "111"
            },
            "2017-07-14T10:40:00.000+08:00":
            {
                "as": "INT",
                "alias": "2017-07-14T10:40:00.000+08:00"
            },
            "10.16":
            {
                "as": "INT UNSIGNED",
                "alias": "10.16"
            },
            "0.3":
            {
                "as": "BIGINT UNSIGNED",
                "alias": "0.3"
            }
        },
        "model":
        {
            "tags":
            [
                "1"
            ],
            "columns":
            [
                "7"
            ]
        }
    }
    }));
    dbg!(&parser);
}
