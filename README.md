# rgetusers

#### By Paul J Barrett

List Aws Users with Arn and password_last_used

Pipe seperated and header row.

I built this to easily document users for security audits. From quarter to quarter you would compare the previous version with the current output to determine the changes and whether the accounts are being used.

Must have aws cli installed to run `aws configure` or have created the default config for aws with keys and region set.

Based on examples in https://github.com/awslabs/aws-sdk-rust

Other packages of note: aws-smithy-types needed to format date. aws_smithy_types::date_time::Format;

If someone could tell me a better way to handle the password_last_used I would appreciate it.
