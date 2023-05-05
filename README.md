# open_dp100
DP100 power supplier from alientek

This project aim to make a free libarary that can communication with DP100 under any Operating System.

First is MacOs then linux, last is windows( or there won't be,because the official has provides one written in '.NET' ).

## links
(Chinese)
模块使用资料 

百度网盘-链接： https://pan.baidu.com/s/1BYMXIUfhYMclGUNLVKNv9Q
提取码：wlf0

## Protocol
see DP100_Protocol.md for protocol reverse engineering

## Build
Steps
1. Download rust of you platform
2. cd this project dir
3. cargo build --release

A bin called 'cli' is under `target/release/`

This is the cli interface of this project

## CLI usage
detail see `cli -h`

3 sub-commands is supported
1. `ls` : list connected DP100s
2. `status` : list DP100
3. `set` : change DP100 settings

### Examples
- List current DP100s that connected

    ```cli ls```

- List first DP100 device's status

    ```cli status```

- List 2nd DP100 device's status

    ```cli status -d1```

- Set first DP100 Output On

    ```cli set on```

- Set DP100 complex 1

    ```cli set -d 1 config=5 v=12.00 ov=30.00 i=1.00 oc=2.00    on```

    set second device switch to config 5,then edit the output value,at last turn it on

- Set DP100 complex 2

    ```cli set config=5 on```

    Switch to config 5 and turn on

## Library

WIP

## Compatablity
|Platform | Status | Note |
| -- | -- |--|
|Windows | Edit needs | There seemds a bug inside current version of `hidapi crate` under Windows,write function not sending all data,and first byte is missing.Edit is needed. |
|Linux| Not Tested | linux should work fine,but untested |
|MacOs | OK |  |

### Windows problem
When I use `mingw gcc` as a compiler,`hidapi crate` won't send all data as it should. I don't know whether it is a problem only occured on my PC or not.
Some code has to be modified to adapt this.
I hope new `hidapi carate` will fix it soon

Changes `src/libs.rs`
```
diff --git a/src/frame.rs b/src/frame.rs
index 87af8af..9472311 100644
--- a/src/frame.rs
+++ b/src/frame.rs
@@ -95,7 +95,8 @@ pub fn deserialize_in_frame(buffer:&[u8;64],frame:&mut Frame)-
>Result<(), FrameE
 }


-pub fn serialize_out_frame<'a>(frame:&Frame,buffer:&mut [u8;64]){
+pub fn serialize_out_frame<'a>(frame:&Frame,_buffer:&mut [u8;64]){
+    let mut buffer = &mut _buffer[1..];
     buffer[0] = 0xfb;
     buffer[1] = frame.op_code.clone() as u8;
     buffer[2] = 0x00; // serial_num
@@ -263,4 +264,4 @@ impl Operational<1> for OperationResult {
         })
     }

-}
\ No newline at end of file
+}

```

