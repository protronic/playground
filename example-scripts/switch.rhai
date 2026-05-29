for x in [42,123.456,"hi",1,50,999] {
    print(switch x {
        42=>"Answer",
        123.456=>"float",
        "hi"=>"hello",
        999=>"big",
        0..100 if x%2==0=>"even",
        0..100=>"odd",
        _=>type_of(x)
    });
}
