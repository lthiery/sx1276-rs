#include <CppUTest/TestHarness.h>

#include "../helium.h"

uint8_t test_data[] = {0xDE, 0xAD, 0xBE, 0xEF};


TEST_GROUP(LongFiGroup)
{
};

TEST(LongFiGroup, FirstTest)
{
    helium_send(test_data, 4);
    LONGS_EQUAL(1, 2);
}

#include "../board.h"
#include "../radio/radio.h"
#include "../radio/sx1276/sx1276.h"

extern "C" {
    void SX1276Write( uint8_t addr, uint8_t data ){
    }

    uint8_t SX1276Read( uint8_t addr ){
        return 0;
    }

    void SX1276SetChannel( uint32_t freq ){

    }

    void SX1276Init( RadioEvents_t *events ){
    }

    void SX1276SetTxConfig( RadioModems_t modem, int8_t power, uint32_t fdev, 
                        uint32_t bandwidth, uint32_t datarate,
                        uint8_t coderate, uint16_t preambleLen,
                        bool fixLen, bool crcOn, bool FreqHopOn,
                        uint8_t HopPeriod, bool iqInverted, uint32_t timeout ){

    }

    void SX1276SetRxConfig( RadioModems_t modem, uint32_t bandwidth,
                         uint32_t datarate, uint8_t coderate,
                         uint32_t bandwidthAfc, uint16_t preambleLen,
                         uint16_t symbTimeout, bool fixLen,
                         uint8_t payloadLen,
                         bool crcOn, bool FreqHopOn, uint8_t HopPeriod,
                         bool iqInverted, bool rxContinuous ){

    }

    void SX1276SetRx( uint32_t timeout ){

    }

    void SX1276Send( uint8_t *buffer, uint8_t size ){

    }
}
