# dynamo_mapper

A simple library to map Rust structs to/from DynamoDB items

## Getting Started

    use uuid::*;
    use dynamo_mapper::*;

    #[derive(
        DynamoMapper,
        Clone
    )]
    pub struct User {
        pub id: Uuid,
        pub first_name: String,
        pub last_name: String,
        pub creation_timestamp: u128,
        pub is_admin: bool,
    }

    // Now your struct as to_dynamo and from_dynamo methods

    let user = User {
        id: Uuid::new_v4(),
        first_name: "Sherlock",
        last_name: "Holmes,
        creation_timestamp: 1572273181,
        is_admin: false,
    };

    let dynamo_user = user.to_dynamo();
    let user_copy = User::from_dynamo(&dynamo_user);


## Built With

* [Rusoto](https://github.com/rusoto/rusoto) - The AWS SDK for Rust

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/Varkal/dynamo_mapper).

## Authors

* **Romain Moreau** - *Initial work* - [Varkal](https://github.com/Varkal)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Notes

* I'm fairly new to Rust programming. So if you have any recommandations, please fill an issue with your comments !


