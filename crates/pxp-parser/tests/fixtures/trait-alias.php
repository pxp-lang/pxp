<?php

class TestClass {
    use TestTrait {
        test as originalTest;
    };

    use TestTrait2, TestTrait3 {
        TestTrait2::test as trait2Test;
        TestTrait3::test as trait3Test;
    }
}
