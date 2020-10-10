cargo build -p ./ --release --target x86_64-unknown-linux-musl &&

cp ./target/release/ ./lambda && zip lambda.zip lambda && rm lambda &&


#aws lambda create-function --function-name divLambda \
  #--handler doesnt.matter \
  #--zip-file fileb://./lambda.zip \
  #--runtime provided \
  #--role arn:aws:iam::XXXXXXXXXXXXX:role/your_lambda_execution_role \
  #--environment Variables={RUST_BACKTRACE=1} \
  #--tracing-config Mode=Active
