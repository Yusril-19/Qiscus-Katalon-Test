import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('https://www.qiscus.com/multichannel/register')

WebUI.waitForPageLoad(1.5, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Company Name_company_name'), 'Qiscus 1')

WebUI.selectOptionByValue(findTestObject('Page_Qiscus Multichannel Chat/select_Select your industry AutomotiveBanki_298c2c'), 
    'Telecommunications', true)

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Work Email_work_email'), 'qiscus1@gmail.com')

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Phone Number_phone_number'), '08123456789')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Password_password'), 'Gelr1u/q0kY=')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Repeat Password_confirmation'), 'Gelr1u/q0kY=')

WebUI.click(findTestObject('Page_Qiscus Multichannel Chat/button_Register'))

WebUI.navigateToUrl('https://www.qiscus.com/multichannel/register')

WebUI.waitForPageLoad(1.5, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Company Name_company_name'), 'Qiscus 2')

WebUI.selectOptionByValue(findTestObject('Page_Qiscus Multichannel Chat/select_Select your industry AutomotiveBanki_298c2c'), 
    'Telecommunications', true)

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Work Email_work_email'), 'qiscus2@gmail.com')

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Phone Number_phone_number'), '08987654321')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Password_password'), 'FZk3nD3s/f8=')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Repeat Password_confirmation'), 'FZk3nD3s/f8=')

WebUI.click(findTestObject('Page_Qiscus Multichannel Chat/button_Register'))

WebUI.navigateToUrl('https://www.qiscus.com/multichannel/register')

WebUI.waitForPageLoad(1.5, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Company Name_company_name'), 'Qiscus Test')

WebUI.selectOptionByValue(findTestObject('Page_Qiscus Multichannel Chat/select_Select your industry AutomotiveBanki_298c2c'), 
    'Telecommunications', true)

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Work Email_work_email'), 'qiscustest@emailtech.info')

WebUI.setText(findTestObject('Page_Qiscus Multichannel Chat/input_Phone Number_phone_number'), '089876543210')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Password_password'), '0MvknSbVDbffUzAMtJmjdA==')

WebUI.setEncryptedText(findTestObject('Page_Qiscus Multichannel Chat/input_Repeat Password_confirmation'), '0MvknSbVDbffUzAMtJmjdA==')

WebUI.click(findTestObject('Page_Qiscus Multichannel Chat/button_Register'))

WebUI.closeBrowser()

